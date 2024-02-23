#[derive(Debug)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn to_string(&self) -> String {
        match self {
            Role::Admin => "admin".to_string(),
            Role::User => "user".to_string(),
        }
    }
}
