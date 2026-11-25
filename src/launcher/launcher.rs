use std::sync::Arc;

use lighty_launcher::{
    Authenticator, UserProfile,
    auth::{AuthProvider, ExposeSecret, MicrosoftAuth, SecretString},
    event::EventBus,
};
use tokio::sync::Mutex;

use crate::events::{auth::AuthEvent, update::AppEvent};

const SERVICE: &str = "RLauncher";
const ACCOUNT: &str = "microsoft_refresh_token";

pub async fn load_profile(
    auth_guard: Arc<Mutex<Option<MicrosoftAuth>>>,
    event_bus: EventBus,
    tx: tokio::sync::mpsc::UnboundedSender<AppEvent>,
) -> anyhow::Result<UserProfile> {
    let mut guard = auth_guard.lock().await;
    let mut auth = guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Auth not initalized"))?;

    let profile = match load_with_refresh_token(&mut auth, &event_bus).await {
        Ok(profile) => profile,
        Err(_) => {
            auth.set_device_code_callback(move |code, url| {
                let _ = tx.send(AppEvent::Auth(AuthEvent::DeviceCode(
                    code.to_string(),
                    url.to_string(),
                )));
            });

            let profile = auth.authenticate(Some(&event_bus)).await?;
            profile
        }
    };

    save_refresh_token(&profile.provider)?;

    Ok(profile)
}

async fn load_with_refresh_token(
    auth: &mut MicrosoftAuth,
    event_bus: &EventBus,
) -> anyhow::Result<UserProfile> {
    let entry = keyring::Entry::new(SERVICE, ACCOUNT)?;
    let token = entry.get_password()?;
    let rt = SecretString::from(token);
    let profile = auth
        .authenticate_with_refresh_token(&rt, Some(event_bus))
        .await?;
    Ok(profile)
}

// salva a conta no pc para não precisar logar toda vez que entra
fn save_refresh_token(provider: &AuthProvider) -> anyhow::Result<()> {
    if let AuthProvider::Microsoft {
        refresh_token: Some(token),
        ..
    } = provider
    {
        let entry = keyring::Entry::new(SERVICE, ACCOUNT)?;
        entry.set_password(token.expose_secret())?;
    }

    Ok(())
}
