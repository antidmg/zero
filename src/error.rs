pub enum Error {
    DatabaseError(String),
    NotFound(String),
    /// Generic internal server error
    Internal(String),
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Error {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound(err.to_string()),
            _ => Error::Internal(err.to_string()),
        }
    }
}
