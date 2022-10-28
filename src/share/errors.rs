#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Command not found")]
    CommandNotFound,

    #[error("")]
    CommandParsingErr(clap::Error),

    #[error("File Read error: `{0}`, `{1}`")]
    FileReadErr(String, String),

    #[error("Input error: `{0}`")]
    InputErr(String),

    #[error("Listener `{0}` does not exist.")]
    ListenerNotExist(String),

    #[error("Could not lock mutex: `{0}`")]
    LockMutex(String),

    #[error("{0}")]
    ReadlineErr(String),

    #[error("Listener start error: `{0}`:`{1}`")]
    ListenerStartErr(String, u16),

    #[error("Invalid IP Address: `{0}`")]
    InvalidIP(String),

    #[error("Server Connect error:\n{0}")]
    ServerConnectErr(String),

    #[error("{0}")]
    DatabaseErr(sea_orm::DbErr),

    #[error("Invalid UUID: `{0}`")]
    InvalidUUID(String),
}
