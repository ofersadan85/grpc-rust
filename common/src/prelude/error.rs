#[derive(Debug, thiserror::Error)]
pub enum Error {
    Io(#[from] std::io::Error),
    Eyre(#[from] color_eyre::Report),
    Addr(#[from] std::net::AddrParseError),
    Tonic(#[from] tonic::transport::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Eyre(e) => e.fmt(f),
            Self::Addr(e) => e.fmt(f),
            Self::Tonic(e) => e.fmt(f),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
