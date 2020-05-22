// error
//#[macro_use] extern crate failure;

use bson::{
    oid::Error as BsonOidError,
    DecoderError as BsonDecoderError,
    EncoderError as BsonEncoderError
};

use mongodb::{
    error::ErrorKind as MongoErrorKind,
    error::WriteFailure as MongoWriteFailure,
    error::Error as MongoError
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Mongo Error")]
    Mongo(#[from] MongoError),

    #[error("Mongo ErrorKind")]
    MongoKindError(#[from] MongoErrorKind),

    #[error("Error encoding BSON")]
    BsonEncode(#[from] BsonEncoderError),

    #[error("Error decoding BSON")]
    BsonDecode(#[from] BsonDecoderError),

    #[error("Invalid document id")]
    BsonOid(#[from] BsonOidError),
}
/*
impl From<MongoError> for Error {
    fn from(e: MongoError) -> Self {
        Error::Mongo(e)
    }
}

impl From<MongoWriteError> for Error {
    fn from(e: MongoWriteError) -> Self {
        Error::MongoWriteError(e)
    }
}

impl From<BsonDecoderError> for Error {
    fn from(e: BsonDecoderError) -> Self {
        Error::BsonDecode(e)
    }
}

impl From<BsonEncoderError> for Error {
    fn from(e: BsonEncoderError) -> Self {
        Error::BsonEncode(e)
    }
}

impl From<BsonOidError> for Error {
    fn from(e: BsonOidError) -> Self {
        Error::BsonOid(e)
    }
}
*/