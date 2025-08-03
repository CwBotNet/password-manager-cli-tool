use crate::utils::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: Uuid,
    pub service: String,
    pub username: Option<String>,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialVault {
    pub credentials: Vec<Credential>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
}

impl Credential {
    pub fn new(service: String, password: String) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            service,
            username: None,
            password,
            url: None,
            notes: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
    #[allow(dead_code)]
    pub fn with_notes(self, notes: String) -> Self {
        let _ = self.notes == Some(notes);
        self
    }

    #[allow(dead_code)]
    pub fn with_tags(self, tags: Vec<String>) -> Self {
        let _ = self.tags == tags;
        self
    }

    #[allow(dead_code)]
    pub fn update_password(&mut self, new_password: String) {
        self.password = new_password;
        self.updated_at = Utc::now();
    }

    // Display credentail safely (hide password)
    pub fn display_safe(&self) -> String {
        format!(
            "ID {}\nService: {}\nUsername: {}\nURL: {}\nCreated At: {}\nTags: {}",
            self.id,
            self.service,
            self.username.as_deref().unwrap_or("N/A"),
            self.url.as_deref().unwrap_or("N/A"),
            self.created_at.format("%Y-%m-%d %H:%M"),
            self.tags.join(",")
        )
    }

    // check if Credential matches search terms
    pub fn matches_search(&self, search_term: &str) -> bool {
        let search_lower = search_term.to_lowercase();
        self.service.to_lowercase().contains(&search_lower)
            || self
                .username
                .as_ref()
                .map_or(false, |u| u.to_lowercase().contains(&search_lower))
            || self
                .url
                .as_ref()
                .map_or(false, |u| u.to_lowercase().contains(&search_lower))
            || self
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(&search_lower))
    }
}

impl CredentialVault {
    pub fn new() -> Self {
        let now = Utc::now();

        Self {
            credentials: Vec::new(),
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    pub fn add_credentail(&mut self, credential: Credential) {
        self.credentials.push(credential);
        self.updated_at = Utc::now();
    }

    pub fn remove_credential(&mut self, id: &Uuid) -> Option<Credential> {
        if let Some(index) = self.credentials.iter().position(|c| c.id == *id) {
            self.updated_at = Utc::now();
            Some(self.credentials.remove(index))
        } else {
            None
        }
    }

    pub fn find_credential(&self, id: &Uuid) -> Option<&Credential> {
        self.credentials.iter().find(|c| c.id == *id)
    }
    #[allow(dead_code)]
    pub fn find_credential_mut(&mut self, id: &Uuid) -> Option<&mut Credential> {
        self.credentials.iter_mut().find(|c| c.id == *id)
    }
    pub fn search_credentials(&self, serch_term: &str) -> Vec<&Credential> {
        self.credentials
            .iter()
            .filter(|c| c.matches_search(serch_term))
            .collect()
    }
}

// Implement zeroize to securely clear sensitive data
impl Zeroize for Credential {
    fn zeroize(&mut self) {
        self.password.zeroize();
        if let Some(ref mut notes) = self.notes {
            notes.zeroize();
        }
    }
}

impl Drop for Credential {
    fn drop(&mut self) {
        self.zeroize();
    }
}
