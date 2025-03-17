use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path;

pub fn get_config_path () -> &'static str {
    let is_dev = env!("RUST_ENV");
    
    // TODO: add compatibility with other OSs.
    if is_dev == "development" {
        "../data/"
    } else {
        "~/.config/ani-lib/"
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub animes_path: String,
    pub database_path: String,
    pub database_schema: String,
    pub images: String
}

impl Default for Config {
    fn default() -> Self {
        Config {   
            animes_path: join_paths(get_config_path(), "animes/"),
            database_path: join_paths(get_config_path(), "database.db"),
            database_schema: join_paths(get_config_path(), "schema.sql"),
            images: join_paths(get_config_path(), "images")
        }
    }
}


pub fn load_config () -> std::io::Result<Config> {
    let config_path = path::Path::new(get_config_path()).join("config.json");
    
    if fs::exists(config_path.to_str().unwrap())? == false {
        return generate_config();
    }

    let raw_conf = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&raw_conf)?;

    Ok(config)
}

fn generate_config () -> std::io::Result<Config> {
    let config = Config::default();
    let json = serde_json::ser::to_string_pretty(&config)?;
    
    let mut file = fs::File::create(join_paths(&get_config_path(), "config.json"))?;
    file.write(json.as_bytes())?;

    Ok(config)
}

/// this is mainly used to join the config files with the corresponding folder at the ani-core
/// crate (useless to sum it up)
pub fn join_paths (p1: &str, p2: &str) -> String {
    std::path::Path::new(p1).join(p2).to_str().unwrap().to_string()
}
