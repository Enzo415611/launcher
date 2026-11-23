use std::sync::Arc;

use lighty_launcher::{UserProfile, auth::MicrosoftAuth};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AuthState {
    pub is_auth_mode: bool,
    pub profile: Option<UserProfile>,
    pub auth: Arc<Mutex<Option<MicrosoftAuth>>>,
    pub code: String,
    pub url: String,
}

impl Default for AuthState {
    fn default() -> Self {
        let auth = MicrosoftAuth::new(std::env::var("CLIENT_ID").expect("CLIENT_ID Not Found"));
        Self {
            is_auth_mode: false,
            profile: None,
            auth: Arc::new(Mutex::new(Some(auth))),
            code: String::new(),
            url: String::new(),
        }
    }
}

impl std::fmt::Display for AuthState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
