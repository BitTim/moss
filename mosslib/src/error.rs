use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // Std
    #[from]
    Io(std::io::Error),

    // PortableMC
    #[from]
    EnvVar(std::env::VarError),

    #[from]
    MsaAuth(portablemc::msa::AuthError),

    #[from]
    MsaDatabase(portablemc::msa::DatabaseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
