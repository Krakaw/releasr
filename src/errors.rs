use thiserror::Error;
#[derive(Error, Debug)]
pub enum ReleasrError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("sqlite error")]
    SqliteError(#[from] rusqlite::Error),
    #[error("Actix error")]
    ActixError(#[from] actix_web::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
