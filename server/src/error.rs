use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::migrate::MigrateError;

// TODO: figure out this error stuff, I just copied and pasted this for now.
#[derive(Debug)]
pub enum Error {
    // unauthorized request to hub endpoint
    HubAuthError,

    // unauthorized request to client endpoint
    ClientAuthError,

    // status code wrap, probably not needed
    StatusCode(StatusCode),

    // sqlx migration error
    SqlxMigrationError(MigrateError),

    // sqlx error
    SqlxError(sqlx::Error),

    // core didn't respond to message
    CoreRecvError,

    // failed to send message to core
    CoreSendError,

    //
    IoError(std::io::Error),

    // axum error wrap
    AxumError(axum::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SqlxMigrationError(e) => Some(e),
            Error::SqlxError(e) => Some(e),
            Error::IoError(e) => Some(e),
            Error::AxumError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<StatusCode> for Error {
    fn from(s: StatusCode) -> Self {
        Self::StatusCode(s)
    }
}

impl From<MigrateError> for Error {
    fn from(e: MigrateError) -> Self {
        Self::SqlxMigrationError(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::CoreRecvError
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::CoreSendError
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<axum::Error> for Error {
    fn from(e: axum::Error) -> Self {
        Self::AxumError(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response: Response = match self {
            Self::StatusCode(s) => s,
            Self::HubAuthError | Self::ClientAuthError => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response();
        response.extensions_mut().insert(self);
        response
    }
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}