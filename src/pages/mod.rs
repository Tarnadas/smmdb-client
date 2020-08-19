mod init;
mod save;

pub use init::InitPage;
pub use save::SavePage;

pub enum Page {
    Init(InitPage),
    Save(SavePage),
}
