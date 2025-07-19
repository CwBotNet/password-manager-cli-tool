use crate::utils::{DateTime,Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Credential{
    pub service: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub notes :Option<String>,
}
