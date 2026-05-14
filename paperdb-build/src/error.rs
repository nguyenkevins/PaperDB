use thiserror::Error;

#[derive(Debug, Error)]
pub enum InsertError {
    #[error("field '{0}' has wrong type")]
    TypeMismatch(String),
    #[error("required field '{0}' is missing")]
    MissingRequiredField(String),
    #[error("field '{0}' is not defined in model")]
    UnknownField(String),
    #[error("duplicate record with id {0}")]
    DuplicateRecord(u64),
    #[error("collection '{0}' not found")]
    CollectionNotFound(String),
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("field '{0}' is not defined in model")]
    UnknownField(String),
    #[error("collection '{0}' not found")]
    CollectionNotFound(String),
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("collection '{0}' already exists")]
    CollectionAlreadyExists(String),
    #[error("collection '{0}' not found")]
    CollectionNotFound(String),
}
