use rand::Rng;

/// Generate a new session token
/// based on cheatsheet from OWASP on session management
/// use CSRPNG to generate a random token with a size of at least 128 bits
/// and ensure that the token is unique
///
/// # Returns
/// A session token
///
/// # Example
/// ```
/// use api_services::redis::services::generate_session_token;
///
/// let token = generate_session_token();
/// ```
pub fn generate_session_token() -> String {
    let mut rng = rand::thread_rng();
    rng.gen::<u128>().to_string()
}
