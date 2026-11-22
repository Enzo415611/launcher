use std::sync::Arc;

use lighty_launcher::{UserProfile, auth::MicrosoftAuth};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AuthState {
    pub profile: Option<UserProfile>,
    pub auth: Arc<Mutex<Option<MicrosoftAuth>>>,
    pub code: String,
    pub url: String,
}

impl Default for AuthState {
    fn default() -> Self {
        let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID Not Found");
        println!("{}", client_id);
        let auth = MicrosoftAuth::new(client_id);
        Self {
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
