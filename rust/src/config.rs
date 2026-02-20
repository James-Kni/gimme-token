use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Profile {
    pub username: String,
    pub password: String,
    pub aws_region: String,
    pub aws_user_pool: String,
    pub aws_client_id: String,
}

#[derive(Deserialize, Default)]
struct PartialProfile {
    username: Option<String>,
    password: Option<String>,
    #[serde(rename = "awsRegion")]
    aws_region: Option<String>,
    #[serde(rename = "awsUserPool")]
    aws_user_pool: Option<String>,
    #[serde(rename = "awsClientId")]
    aws_client_id: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    #[serde(default = "default_profile_name")]
    default_profile: String,
    #[serde(default)]
    profile: HashMap<String, PartialProfile>,
}

fn default_profile_name() -> String {
    "default".to_string()
}

fn config_path() -> PathBuf {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        PathBuf::from(xdg).join("gimme-token/config.toml")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".config/gimme-token/config.toml")
    } else {
        eprintln!("[ERROR] Unable to find config location");
        std::process::exit(0);
    }
}

fn merge(base: &PartialProfile, over: Option<&PartialProfile>) -> PartialProfile {
    let default_partial = PartialProfile::default();
    let o = over.unwrap_or(&default_partial);
    PartialProfile {
        username: o.username.clone().or_else(|| base.username.clone()),
        password: o.password.clone().or_else(|| base.password.clone()),
        aws_region: o.aws_region.clone().or_else(|| base.aws_region.clone()),
        aws_user_pool: o
            .aws_user_pool
            .clone()
            .or_else(|| base.aws_user_pool.clone()),
        aws_client_id: o
            .aws_client_id
            .clone()
            .or_else(|| base.aws_client_id.clone()),
    }
}

pub fn load_profile(profile_key: Option<&str>) -> Profile {
    let path = config_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&path, "");
        println!("Empty config file created at {}", path.display());
        println!("You should probably edit this.");
        std::process::exit(0);
    }

    let content = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("[ERROR] Unable to read config: {e}");
        std::process::exit(0);
    });

    let config: Config = toml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(0);
    });

    if let Some(key) = profile_key {
        if !config.profile.contains_key(key) {
            println!("Profile '{key}' is not defined");
            for name in config.profile.keys() {
                println!("- {name}");
            }
            std::process::exit(0);
        }
    }

    let default = &PartialProfile::default();
    let base = config
        .profile
        .get(&config.default_profile)
        .unwrap_or(default);
    let over = profile_key.and_then(|k| config.profile.get(k));
    let merged = merge(base, over);

    let missing: Vec<&str> = [
        merged.username.is_none().then_some("username"),
        merged.password.is_none().then_some("password"),
        merged.aws_region.is_none().then_some("awsRegion"),
        merged.aws_user_pool.is_none().then_some("awsUserPool"),
        merged.aws_client_id.is_none().then_some("awsClientId"),
    ]
    .into_iter()
    .flatten()
    .collect();

    if !missing.is_empty() {
        eprintln!("Missing required fields: {}", missing.join(", "));
        std::process::exit(0);
    }

    Profile {
        username: merged.username.unwrap(),
        password: merged.password.unwrap(),
        aws_region: merged.aws_region.unwrap(),
        aws_user_pool: merged.aws_user_pool.unwrap(),
        aws_client_id: merged.aws_client_id.unwrap(),
    }
}
