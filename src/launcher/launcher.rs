use std::sync::Arc;

use anyhow::Error;
use lighty_launcher::{
    Authenticator, UserProfile,
    auth::{AuthProvider, ExposeSecret, MicrosoftAuth, SecretString},
    event::EventBus,
};
use tokio::sync::Mutex;

const SERVICE: &str = "VanillaLauncher";
const ACCOUNT: &str = "microsoft_refresh_token";

pub async fn load_profile(
    auth_guard: Arc<Mutex<Option<MicrosoftAuth>>>,
    event_bus: EventBus,
) -> anyhow::Result<UserProfile> {
    // tenta fazer fazer login com uma conta já logada no pc
    if let Ok(mut auth) = auth_guard.try_lock() {
        if let Some(auth) = auth.as_mut() {
            let profile = match load_refresh_token() {
                Some(rt) => match auth
                    .authenticate_with_refresh_token(&rt, Some(&event_bus))
                    .await
                {
                    Ok(profile) => Ok(profile),
                    Err(err) => {
                        eprintln!("{}", err);
                        Err(err.into())
                    }
                },
                None => Ok(auth.authenticate(Some(&event_bus)).await?),
            };

            // salva a conta logada
            if let Ok(profile) = &profile {
                save_refresh_token(&profile.provider)?;
            }
            return profile;
        } else {
            Err(Error::msg(""))
        }
    } else {
        Err(Error::msg(""))
    }
}

// tenta carregar a conta caso ja tenha logada no pc
fn load_refresh_token() -> Option<SecretString> {
    let entry = keyring::Entry::new(SERVICE, ACCOUNT).ok()?;
    let token = entry.get_password().ok()?;
    Some(SecretString::from(token))
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
