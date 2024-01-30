#[derive(Debug)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn to_string(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
        }
    }
}
