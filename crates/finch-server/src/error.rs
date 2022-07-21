use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
}

pub type Result<T> = std::result::Result<T, Error>;
