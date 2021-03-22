mod init;
mod save;
mod settings;

pub use init::InitPage;
pub use save::SavePage;
pub use settings::SettingsPage;

#[derive(Clone, Debug)]
pub enum Page {
    Init(InitPage),
    Save(Box<SavePage>),
    Settings(SettingsPage),
}
