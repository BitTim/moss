use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
