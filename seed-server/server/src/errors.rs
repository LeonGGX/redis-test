// shared/src/errors/errors.rs

use bson::{
    oid::Error as BsonOidError, DecoderError as BsonDecoderError, EncoderError as BsonEncoderError,
};

use mongodb::{error::Error as MongoError, error::ErrorKind as MongoErrorKind};

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
