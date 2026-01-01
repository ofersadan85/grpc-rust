#[derive(Debug, thiserror::Error)]
pub enum Error {
    Unknown,
    Io(#[from] std::io::Error),
    Eyre(#[from] color_eyre::Report),
    Addr(#[from] std::net::AddrParseError),
    TonicError(#[from] tonic::transport::Error),
    TonicStatus(#[from] tonic::Status),
    Json(#[from] serde_json::Error),
    InvalidUri(#[from] http::uri::InvalidUri),
    JoinError(#[from] tokio::task::JoinError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "An unknown error occurred"),
            Self::Io(e) => e.fmt(f),
            Self::Eyre(e) => e.fmt(f),
            Self::Addr(e) => e.fmt(f),
            Self::TonicError(e) => e.fmt(f),
            Self::TonicStatus(e) => e.fmt(f),
            Self::Json(e) => e.fmt(f),
            Self::InvalidUri(e) => e.fmt(f),
            Self::JoinError(e) => e.fmt(f),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<()> for Error {
    fn from((): ()) -> Self {
        Self::Unknown
    }
}
