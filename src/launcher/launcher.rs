use std::sync::Arc;

use anyhow::Error;
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
    let auth = guard
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Auth not initalized"))?;

    let profile = match load_refresh_token() {
        Some(rt) => match auth
            .authenticate_with_refresh_token(&rt, Some(&event_bus))
            .await
        {
            Ok(profile) => Ok(profile),
            Err(err) => Err(err),
        },
        None => {
            auth.set_device_code_callback(move |code, url| {
                let _ = tx.send(AppEvent::Auth(AuthEvent::DeviceCode(
                    code.to_string(),
                    url.to_string(),
                )));
            });

            let profile = auth.authenticate(Some(&event_bus)).await?;
            Ok(profile)
        }
    };
    if let Ok(profile) = &profile {
        save_refresh_token(&profile.provider)?;
    }

    profile.map_err(|err| anyhow::Error::msg(err.to_string()))
}

// if let Ok(mut auth) = auth_guard.try_lock() {
//     if let Some(auth) = auth.as_mut() {
//         let profile = match load_refresh_token() {
//             // tenta fazer fazer login com uma conta já logada no pc
//             Some(rt) => match auth
//                 .authenticate_with_refresh_token(&rt, Some(&event_bus))
//                 .await
//             {
//                 Ok(profile) => Ok(profile),
//                 Err(err) => {
//                     eprintln!("{}", err);
//                     Ok(auth.authenticate(Some(&event_bus)).await?)
//                 }
//             },
//             // caso não tenha uma conta ja logada
//             None => {
//                 println!("esperar micro");
//                 Ok(auth.authenticate(Some(&event_bus)).await?)
//             }
//         };
//
//         // salva a conta logada e retorna o profile
//         if let Ok(profile) = &profile {
//             println!("save profile");
//             save_refresh_token(&profile.provider)?;
//         }
//         return profile;
//     } else {
//         Err(Error::msg(""))
//     }
// } else {
//     Err(Error::msg(""))
// }

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
