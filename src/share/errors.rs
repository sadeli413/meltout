#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Command not found")]
    CommandNotFound,

    #[error("")]
    CommandParsingErr(clap::Error),

    #[error("Input error: `{0}`")]
    InputErr(String),

    #[error("Listener `{0}` does not exist.")]
    ListenerNotExist(String),

    #[error("Could not lock mutex: `{0}`")]
    LockMutex(String),

    #[error("{0}")]
    ReadlineErr(String),

    #[error("Server start error: `{0}`")]
    ListenerStartErr(String),

    #[error("Invalid IP Address: `{0}`")]
    InvalidIP(String),

    #[error("Server Connect error:\n{0}")]
    ServerConnectErr(String),

    #[error("{0}")]
    DatabaseErr(sea_orm::DbErr)
}
