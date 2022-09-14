#[derive(Debug)]
pub enum Error {
    ConfigurationError,
    SessionError,
    TungsteniteError(tokio_tungstenite::tungstenite::Error),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::TungsteniteError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client web socket error!")
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Self::ConfigurationError => "Attempted to connect without syncing configuration from server.",
            Self::SessionError => "Attempted to connect without valid session.",
            Self::TungsteniteError(_) => "Tungstenite error",
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TungsteniteError(e) => Some(e),
            _ => None
        }
    }
}