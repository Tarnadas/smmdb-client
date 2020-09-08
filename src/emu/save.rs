use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct EmuSave {
    display_name: String,
    location: PathBuf,
    emu_type: EmuType,
}

#[derive(Clone, Debug)]
pub enum EmuType {
    Yuzu,
    Ryujinx,
}

impl EmuSave {
    pub fn new(display_name: String, location: PathBuf, emu_type: EmuType) -> EmuSave {
        EmuSave {
            display_name,
            location,
            emu_type,
        }
    }

    pub fn get_display_name(&self) -> &String {
        &self.display_name
    }

    pub fn get_location(&self) -> &PathBuf {
        &self.location
    }
}

impl Display for EmuSave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {:?}", self.emu_type, self.location)
    }
}
