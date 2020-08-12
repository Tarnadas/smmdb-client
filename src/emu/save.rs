use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct EmuSave {
    location: PathBuf,
    emu_type: EmuType,
    has_smm2_save: bool,
}

#[derive(Clone, Debug)]
pub enum EmuType {
    Yuzu,
    Ryujinx,
}

impl EmuSave {
    pub fn new(location: PathBuf, emu_type: EmuType) -> EmuSave {
        let mut smm2_location = location.clone();
        smm2_location.push(EmuSave::get_smm2_subpath(&emu_type));
        let has_smm2_save = smm2_location.exists();
        EmuSave {
            location,
            emu_type,
            has_smm2_save,
        }
    }

    pub fn get_location(&self) -> &PathBuf {
        &self.location
    }

    pub fn get_smm2_location(&self) -> PathBuf {
        let mut location = self.location.clone();
        location.push(EmuSave::get_smm2_subpath(&self.emu_type));
        location
    }

    fn get_smm2_subpath(emu_type: &EmuType) -> PathBuf {
        match emu_type {
            EmuType::Yuzu => {
                "nand/user/save/0000000000000000/FDD588AE7826C7A9A70AE93C12A4E9CE/01009B90006DC000"
                    .into()
            }
            // TODO this looks wrong
            EmuType::Ryujinx => "bis/user/save/0000000000000001/0".into(),
        }
    }
}

impl Display for EmuSave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {:?}", self.emu_type, self.location)
    }
}
