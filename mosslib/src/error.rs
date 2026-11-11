use std::path::PathBuf;

use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no account found with username {username}")]
    NoAccountWithUsernameFound { username: String },

    #[error("no account found with uuid {uuid}")]
    NoAccountWithUuidFound { uuid: Uuid },

    // Std
    #[error("failed to access {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // PortableMC
    #[error("Microsoft authentication failed")]
    MsaAuth(#[from] portablemc::msa::AuthError),

    #[error("account database error")]
    MsaDatabase(#[from] portablemc::msa::DatabaseError),
}
