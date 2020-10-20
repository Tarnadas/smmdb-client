use crate::components::SaveButton;

use anyhow::Result;
use std::{
    collections::HashSet,
    fmt::Write,
    fs::{read_dir, File},
    io::Read,
    num::ParseIntError,
    path::PathBuf,
};

mod save;

pub use save::*;

pub fn guess_emu_dir() -> Result<Vec<SaveButton>> {
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
        )?;
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_dir,
            &ryujinx_guesses,
            EmuType::Ryujinx,
            is_ryujinx_dir,
        )?;
    }
    if let Some(config_dir) = dirs::config_dir() {
        guess_dir(
            &mut dirs,
            &mut found_paths,
            config_dir.clone(),
            &yuzu_guesses,
            EmuType::Yuzu,
            is_yuzu_dir,
        )?;
        guess_dir(
            &mut dirs,
            &mut found_paths,
            config_dir,
            &ryujinx_guesses,
            EmuType::Ryujinx,
            is_ryujinx_dir,
        )?;
    }
    if let Some(data_local_dir) = dirs::data_local_dir() {
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_local_dir.clone(),
            &yuzu_guesses,
            EmuType::Yuzu,
            is_yuzu_dir,
        )?;
        guess_dir(
            &mut dirs,
            &mut found_paths,
            data_local_dir,
            &ryujinx_guesses,
            EmuType::Ryujinx,
            is_ryujinx_dir,
        )?;
    }
    Ok(dirs)
}

fn guess_dir(
    dirs: &mut Vec<SaveButton>,
    found_paths: &mut HashSet<PathBuf>,
    dir: PathBuf,
    guesses: &[&str],
    emu_type: EmuType,
    is_emu_type: fn(PathBuf) -> bool,
) -> Result<()> {
    for guess in guesses.iter() {
        let mut current_dir = dir.clone();
        current_dir.push(guess);
        if current_dir.as_path().exists() && is_emu_type(current_dir.clone()) {
            match emu_type {
                EmuType::Yuzu => {
                    current_dir.push("nand/user/save/0000000000000000");
                    for entry in read_dir(current_dir.clone())? {
                        let entry = entry?;
                        let mut path = entry.path();
                        if path.is_dir() {
                            path.push("01009B90006DC000");
                            if path.exists() && found_paths.get(&path).is_none() {
                                found_paths.insert(path.clone());
                                let display_name =
                                    format!("[{:?}] {}", &emu_type, dir.to_string_lossy());
                                dirs.push(SaveButton::new(display_name, path, emu_type.clone()));
                            }
                        }
                    }
                }
                EmuType::Ryujinx => {
                    let display_name =
                        format!("[{:?}] {}", &emu_type, current_dir.to_string_lossy());
                    let mut imkvdb_path = current_dir.clone();
                    imkvdb_path.push("bis/system/save/8000000000000000/0/imkvdb.arc");
                    current_dir.push("bis/user/save");
                    dbg!(&current_dir);

                    if imkvdb_path.exists() {
                        let mut imkvdb = File::open(imkvdb_path)?;
                        let mut buffer = vec![];
                        imkvdb.read_to_end(&mut buffer)?;
                        let mut game_id = decode_hex("01009B90006DC000")?;
                        game_id.reverse();
                        dbg!(&game_id);

                        let imen_header_size = 0xC;
                        for chunk in buffer[0xC..].chunks(0x80 + imen_header_size) {
                            dbg!(&chunk[imen_header_size..imen_header_size + 8]);
                            if game_id != chunk[imen_header_size..imen_header_size + 8].to_vec() {
                                continue;
                            }
                            let mut save_data_chunks =
                                chunk[imen_header_size + 0x40..imen_header_size + 0x48].to_vec();
                            save_data_chunks.reverse();
                            let save_data_id = encode_hex(&save_data_chunks)?;
                            dbg!(&save_data_id);

                            let mut path = current_dir.clone();
                            path.push(save_data_id);
                            path.push("0");

                            if path.exists() && found_paths.get(&path).is_none() {
                                found_paths.insert(path.clone());
                                dirs.push(SaveButton::new(
                                    display_name.clone(),
                                    path,
                                    emu_type.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
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

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn encode_hex(bytes: &[u8]) -> Result<String> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b)?;
    }
    Ok(s)
}
