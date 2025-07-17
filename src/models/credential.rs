#[derive(Debug, Clone)]
pub struct Credential{
    pub service: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub notes :Option<String>,
}
