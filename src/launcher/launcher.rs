use lighty_launcher::{
    Authenticator, Launch, Loader, UserProfile, VersionBuilder,
    auth::{AuthProvider, ExposeSecret, MicrosoftAuth, SecretString},
    event::EventBus,
};

use crate::AppState;

const SERVICE: &str = "VanillaLauncher";
const ACCOUNT: &str = "microsoft_refresh_token";

#[derive(Debug)]
pub struct Instance {
    name: String,
    loader: InstanceLoader,
    loader_version: String,
    mc_version: String,
}

#[derive(Debug)]
pub enum InstanceLoader {
    Vanilla,
    Fabric,
}

pub async fn load_profile(event_bus: EventBus) -> anyhow::Result<UserProfile> {
    // passe o client id
    let mut auth = MicrosoftAuth::new("xxxx-x-x-x-x-x--x");

    auth.set_device_code_callback(|code, url| {
        println!("{}", url);
        println!("{}", code);
    });

    // tenta fazer fazer login com uma conta já logada no pc
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
    profile
}

pub async fn load_instance(profile: UserProfile, event_bus: &EventBus) -> anyhow::Result<()> {
    // criar uma instancia passando o nome, loader, loader versions(vanilla não precisa) e game version
    let mut instance = VersionBuilder::new("instance name", Loader::Vanilla, "", "1.21.1");
    instance
        .launch(&profile, lighty_launcher::JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;
    Ok(())
}

async fn event_bus(event_bus: EventBus) {
    let mut receiver = event_bus.subscribe();
    while let Ok(event) = receiver.next().await {
        match event {
            lighty_launcher::event::Event::InstanceExited(_) => {
                break;
            }
            _ => {}
        }
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
