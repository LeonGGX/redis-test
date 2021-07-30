// src/errors.rs

use sqlx::postgres::PgError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustError {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(PgError),
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] sqlx::Error),
    #[error("sqlx postgres erreur")]
    PgSqlxError(#[from] sqlx::postgres::PgError),
}

impl warp::reject::Reject for CustError {}
