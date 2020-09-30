use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir, read, write, File},
    io::Write,
    path::PathBuf,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub apikey: Option<String>,
}

impl Settings {
    pub fn load() -> Result<Settings> {
        let settings_path = Settings::get_path()?;
        let settings = if let Ok(settings) = read(settings_path.clone()) {
            serde_json::from_slice(&settings)?
        } else {
            let settings = Settings::default();
            let mut file = File::create(settings_path)?;
            file.write_all(serde_json::to_string(&settings)?.as_bytes())?;
            settings
        };
        Ok(settings)
    }

    fn get_path() -> Result<PathBuf> {
        let mut config_dir = if let Some(config_dir) = dirs::config_dir() {
            config_dir
        } else {
            dirs::data_dir().expect("Could not initialize app directory")
        };
        config_dir.push("smmdb-client");
        if !config_dir.exists() {
            create_dir(config_dir.clone())?;
        }
        let mut settings_path = config_dir.clone();
        settings_path.push("settings.json");
        Ok(settings_path)
    }

    pub fn save(&self) -> Result<()> {
        let settings_path = Settings::get_path()?;
        let settings = serde_json::to_string(&self)?;
        write(settings_path, settings)?;
        Ok(())
    }
}
