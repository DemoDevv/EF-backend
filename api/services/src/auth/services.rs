use std::sync::Arc;

use actix_web::{dev::ServiceRequest, web};

use api_caches::access_refresh_tokens::{AccessRefreshTokensCache, UserMetaData};
use api_configs::config::Config;
use api_db::{models::user::User, repository::UserRepository};
use api_errors::{ServiceError, ServiceErrorType};
use api_types::user::{InputUser, NewUser, RefreshableUser};

use argon2::PasswordHash;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use time::{Duration, OffsetDateTime};

use crate::oauth::fetch_google_user_info;

use super::{
    claims::TokenClaims,
    errors::AuthentificationError,
    helpers::{hash_password, verify_password},
    types::Tokens,
};

#[derive(Clone)]
pub struct AuthService<U: UserRepository, C: AccessRefreshTokensCache> {
    users_repository: Arc<U>,
    access_refresh_tokens_cache: Arc<C>,
}

impl<U: UserRepository, C: AccessRefreshTokensCache> AuthService<U, C> {
    pub fn new(users_repository: Arc<U>, access_refresh_tokens_cache: Arc<C>) -> Self {
        Self {
            users_repository,
            access_refresh_tokens_cache,
        }
    }

    pub async fn login(
        &self,
        user_json: InputUser,
        config: &Config,
    ) -> Result<Tokens, ServiceError> {
        let user = self
            .users_repository
            .get_user_by_email(&user_json.email)
            .await;

        if let Err(_) = user {
            // si il y a une erreur avec la base de données
            return Err(ServiceError {
                message: Some("Authentification failed".to_string()),
                error_type: ServiceErrorType::BadAuthentification,
            });
        }

        let user = user.unwrap();

        if user.password.is_none() {
            return Err(ServiceError {
                message: Some("Authentification failed (consider using oauth)".to_string()),
                error_type: ServiceErrorType::BadAuthentification,
            });
        }

        let password = user.password.as_ref().unwrap();

        let parsed_hash = PasswordHash::new(password)
            .map_err(|err| AuthentificationError::from(err))
            .map_err(|err| ServiceError::from(err))?;

        if let Err(err) = verify_password(&user_json.password, &parsed_hash) {
            return Err(ServiceError::from(err));
        }

        self.generate_and_save_tokens(&user, config).await
    }

    pub async fn register(
        &self,
        user_json: InputUser,
        config: &Config,
    ) -> Result<Tokens, ServiceError> {
        let created_user = match self
            .users_repository
            .get_user_by_email(&user_json.email)
            .await
        {
            Ok(db_user) => {
                if db_user.password.is_some() {
                    return Err(ServiceError {
                        message: Some("User already exists".to_string()),
                        error_type: ServiceErrorType::BadAuthentification,
                    });
                }

                // Si l'utilisateur a un compte Google mais pas de mot de passe
                if db_user.google_id.is_some() {
                    let hash = hash_password(&user_json.password).map_err(ServiceError::from)?;

                    let mut cloned_db_user = db_user.clone();
                    cloned_db_user.password = Some(hash);

                    self.users_repository
                        .update(db_user.id, &cloned_db_user)
                        .await?
                } else {
                    return Err(ServiceError {
                        message: Some("Invalid user state".to_string()),
                        error_type: ServiceErrorType::BadAuthentification,
                    });
                }
            }
            Err(e) => {
                if e.error_type == ServiceErrorType::DatabaseError {
                    return Err(ServiceError {
                        message: Some("Error getting user".to_string()),
                        error_type: ServiceErrorType::InternalServerError,
                    });
                }

                // Créer un nouvel utilisateur
                let hash = hash_password(&user_json.password).map_err(ServiceError::from)?;

                self.users_repository
                    .create(&NewUser {
                        pseudo: generate_random_pseudo(),
                        first_name: None,
                        last_name: None,
                        email: user_json.email.to_string(),
                        password: Some(hash),
                        google_id: None,
                    })
                    .await?
            }
        };

        self.generate_and_save_tokens(&created_user, config).await
    }

    pub async fn handle_oauth_connection(
        &self,
        google_access_token: &str,
        config: &Config,
    ) -> Result<Tokens, ServiceError> {
        // get google profile information
        let google_profile = fetch_google_user_info(google_access_token).await?;

        // check if the user already exists
        if let Ok(user) = self
            .users_repository
            .get_user_by_google_id(&google_profile.id)
            .await
        {
            return self.generate_and_save_tokens(&user, config).await;
        }

        if let Ok(user) = self
            .users_repository
            .get_user_by_email(&google_profile.email)
            .await
        {
            if user.google_id.is_none() {
                return Err(ServiceError {
                    message: Some("An account already exist with this email. Please use the default auth way for link your google account.".to_string()),
                    error_type: ServiceErrorType::BadAuthentification,
                });
            }
        }

        // create a new user
        let created_user = self
            .users_repository
            .create(&NewUser {
                pseudo: google_profile.name,
                first_name: Some(google_profile.given_name),
                last_name: Some(google_profile.family_name),
                email: google_profile.email,
                password: None,
                google_id: Some(google_profile.id),
            })
            .await?;

        self.generate_and_save_tokens(&created_user, config).await
    }

    pub async fn refresh_tokens(
        &self,
        user_payload: RefreshableUser,
        config: &Config,
    ) -> Result<Tokens, ServiceError> {
        let meta_data_user = self
            .access_refresh_tokens_cache
            .get_meta_data_users_by_refresh_token(&user_payload.refresh_token)
            .await?;

        if meta_data_user.email != user_payload.email {
            return Err(ServiceError {
                message: Some("User meta data does not match with registered data".to_string()),
                error_type: ServiceErrorType::UnAuthorized,
            }
            .into());
        }

        let new_refresh_token = generate_refresh_token();

        let tokens = Tokens {
            access_token: create_valid_token(config, meta_data_user.id.parse::<i32>().unwrap())?,
            refresh_token: new_refresh_token.clone(),
        };

        // make rotation of the refresh token and invalidate it
        self.access_refresh_tokens_cache
            .invalidate_and_save_token(&user_payload.refresh_token, &new_refresh_token)
            .await?;

        Ok(tokens)
    }

    async fn generate_and_save_tokens(
        &self,
        user: &User,
        config: &Config,
    ) -> Result<Tokens, ServiceError> {
        // Générer les tokens
        let tokens = Tokens {
            access_token: create_valid_token(config, user.id)?,
            refresh_token: generate_refresh_token(),
        };

        // Sauvegarder les métadonnées utilisateur
        let user_meta_data = UserMetaData {
            id: user.id.to_string(),
            email: user.email.clone(),
        };

        self.access_refresh_tokens_cache
            .save_refresh_token(&tokens.refresh_token, user_meta_data)
            .await
            .map_err(ServiceError::from)?;

        Ok(tokens)
    }
}

pub fn validate_token(
    req: &ServiceRequest,
    token: &str,
) -> Result<TokenData<TokenClaims>, ServiceError> {
    let config = req.app_data::<web::Data<Config>>().unwrap();

    let claims = decode_token(config.clone(), token);

    match claims {
        Ok(token_data) => Ok(token_data),
        Err(_) => Err(ServiceError {
            message: Some("le token n'a pas pu être décodé.".to_string()),
            error_type: ServiceErrorType::BadAuthentification,
        }),
    }
}

pub fn create_valid_token(config: &Config, user_id: i32) -> Result<String, ServiceError> {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::minutes(config.jwt_expired_in);

    let claims = TokenClaims::new(user_id, iat, exp);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );

    match token {
        Ok(token_encoded) => Ok(token_encoded),
        Err(_) => Err(ServiceError {
            message: Some("encode failed".to_string()),
            error_type: ServiceErrorType::InternalServerError,
        }),
    }
}

pub fn decode_token(
    config: web::Data<Config>,
    token: &str,
) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
    // suppresion du leeway pour pouvoir exp le token avant 1 minute
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 1;

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    )
}

pub fn generate_refresh_token() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn generate_random_pseudo() -> String {
    format!("Newbie{}", uuid::Uuid::new_v4().to_string())
}
