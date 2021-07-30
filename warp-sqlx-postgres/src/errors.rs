// src/errors.rs

// A faire ne pas utiliser maintenant
// traiter les erreurs actix avec une erreur spéciale ...
// etc ...


use thiserror::Error;
use sqlx::Error;
use sqlx::postgres::PgError;
use std::io;

#[derive(Error, Debug)]
pub enum CustError {
    #[error("I/O erreur")]
    InOutError(#[from] io::Error),
    #[error("sqlx erreur")]
    SqlxError(#[from] sqlx::Error),
    #[error("sqlx postgres erreur")]
    PgSqlxError(#[from] sqlx::postgres::PgError),
}