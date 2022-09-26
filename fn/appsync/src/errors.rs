use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ServerError {
    #[error("could not serialize JSON")]
    Disconnect(#[from] serde_json::Error),
    #[error("error creating response")]
    Response(#[from] http::Error),
}
