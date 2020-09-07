use crate::components::SaveButton;

use std::{collections::HashSet, path::PathBuf};

mod save;

pub use save::*;

pub fn guess_emu_dir() -> Vec<SaveButton> {
    let mut dirs = vec![];
    let mut found_paths: HashSet<PathBuf> = HashSet::new();
    let yuzu_guesses = ["yuzu", "yuzu-emu"];
    let ryujinx_guesses = ["Ryujinx"];
    if let Some(data_dir) = dirs::data_dir() {
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_dir.clone(),
            &yuzu_guesses,
            EmuType::Yuzu,
            is_yuzu_dir,
        );
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_dir,
            &ryujinx_guesses,
            EmuType::Ryujinx,
            is_ryujinx_dir,
        );
    }
    if let Some(config_dir) = dirs::config_dir() {
        guess_dir(
            &mut dirs,
            &mut found_paths,
            config_dir.clone(),
            &yuzu_guesses,
            EmuType::Yuzu,
            is_yuzu_dir,
        );
        guess_dir(
            &mut dirs,
            &mut found_paths,
            config_dir,
            &ryujinx_guesses,
            EmuType::Ryujinx,
            is_ryujinx_dir,
        );
    }
    if let Some(data_local_dir) = dirs::data_local_dir() {
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_local_dir.clone(),
            &yuzu_guesses,
            EmuType::Yuzu,
            is_yuzu_dir,
        );
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_local_dir,
            &ryujinx_guesses,
            EmuType::Ryujinx,
            is_ryujinx_dir,
        );
    }
    dirs
}

fn guess_dir(
    dirs: &mut Vec<SaveButton>,
    found_paths: &mut HashSet<PathBuf>,
    dir: PathBuf,
    guesses: &[&str],
    emu_type: EmuType,
    is_emu_type: fn(PathBuf) -> bool,
) {
    for guess in guesses.iter() {
        let mut dir = dir.clone();
        dir.push(guess);
        if dir.as_path().exists() && is_emu_type(dir.clone()) && found_paths.get(&dir).is_none() {
            found_paths.insert(dir.clone());
            dirs.push(SaveButton::new(dir, emu_type.clone()));
        }
    }
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
