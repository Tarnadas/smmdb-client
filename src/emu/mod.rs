use crate::components::SaveButton;

use std::path::PathBuf;

mod save;

pub use save::*;

pub fn guess_emu_dir() -> Vec<SaveButton> {
    let mut dirs = vec![];
    if let Some(data_dir) = dirs::data_dir() {
        let guesses = ["yuzu", "yuzu-emu"];
        for guess in guesses.iter() {
            let mut data_dir = data_dir.clone();
            data_dir.push(guess);
            if data_dir.as_path().exists() && is_yuzu_dir(data_dir.clone()) {
                dirs.push(SaveButton::new(data_dir, EmuType::Yuzu));
            }
        }
    }
    if let Some(data_dir) = dirs::config_dir() {
        let guesses = ["Ryujinx"];
        for guess in guesses.iter() {
            let mut data_dir = data_dir.clone();
            data_dir.push(guess);
            if data_dir.as_path().exists() && is_ryujinx_dir(data_dir.clone()) {
                dirs.push(SaveButton::new(data_dir, EmuType::Ryujinx));
            }
        }
    }
    dirs
}

pub fn is_yuzu_dir(path: PathBuf) -> bool {
    let mut system_path = path.clone();
    system_path.push("nand");
    system_path.push("system");
    let mut key_path = path;
    key_path.push("keys");
    system_path.as_path().exists() && key_path.as_path().exists()
}

pub fn is_ryujinx_dir(path: PathBuf) -> bool {
    let mut system_path = path.clone();
    system_path.push("system");
    let mut config_path = path;
    config_path.push("Config.json");
    system_path.as_path().exists() && config_path.as_path().exists()
}
