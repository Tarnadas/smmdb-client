use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct Save {
    location: PathBuf,
    emu_type: EmuType,
}

#[derive(Clone, Debug)]
pub enum EmuType {
    Yuzu,
    Ryujinx,
}

impl Save {
    pub fn new(location: PathBuf, emu_type: EmuType) -> Save {
        Save { location, emu_type }
    }

    pub fn get_location(&self) -> &PathBuf {
        &self.location
    }
}

impl Display for Save {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {:?}", self.emu_type, self.location)
    }
}
