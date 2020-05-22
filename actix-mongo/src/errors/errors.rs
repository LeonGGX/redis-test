// error
//#[macro_use] extern crate failure;

use bson::{
    oid::Error as BsonOidError,
    DecoderError as BsonDecoderError,
    EncoderError as BsonEncoderError
};

use mongodb::{
    error::WriteError as MongoWriteError,
    error::Error as MongoError
};

use failure::{Fail};

#[derive(Fail, Debug)]
//#[derive(Debug)]
pub enum Error {
    #[fail(display = "Error: {:?}", _0)]
    Custom(String),

    #[fail(display = "Mongo Error: {}", _0)]
    Mongo(#[cause] MongoError),

    #[fail(display = "Mongo Write Error: {:?}", _0)]
    MongoWriteError(#[cause] MongoWriteError),

    #[fail(display = "Error encoding BSON: {}", _0)]
    BsonEncode(#[cause] BsonEncoderError),

    #[fail(display = "Error decoding BSON: {}", _0)]
    BsonDecode(#[cause] BsonDecoderError),

    #[fail(display = "Invalid document id: {}", _0)]
    BsonOid(#[cause] BsonOidError),
}

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
