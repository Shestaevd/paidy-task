use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Can not find user specified user: ${0}")]
    NoSuchUserError(String),
    #[error("Can not give access to requested resource for user: ${0}")]
    NoPermissionError(String),
    #[error("Can not find auth header")]
    NoAuthHeaderError,
    #[error("Expected auth method is Basic, auth method found :")]
    WrongAuthMethodError(String),
    #[error("Can not decode user from Auth header")]
    UserDecodeError
}

#[derive(Error, Debug)]
pub enum ConfigReadingError {
    #[error("Can not find config in directory: ${0}")]
    WrongPathError(String),
    #[error("Can not parse config due to: ${0}")]
    ParsingError(String)
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Can not create postgres connection pool due to: ${0}")]
    PoolCreationError(String),
    #[error("Can not create postgres transaction due to: ${0}")]
    TransactionCreationError(String),
    #[error("Can not create postgres connection pool due to: ${0}")]
    BadQueryError(String),
    #[error("Can not find value: ${0}")]
    ValueNotFoundError(String)
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP error occurred: {0}")]
    Http(#[from] HttpError),

    #[error("Configuration error occurred: {0}")]
    Config(#[from] ConfigReadingError),

    #[error("Database error occurred: {0}")]
    Database(#[from] DatabaseError),
}