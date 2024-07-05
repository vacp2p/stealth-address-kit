use ark_serialize::SerializationError;

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum CErrorCode {
    NoError = 0,
    SerializationErrorNotEnoughSpace = 1,
    SerializationErrorInvalidData = 2,
    SerializationErrorUnexpectedFlags = 3,
    SerializationErrorIoError = 4,
    InvalidKeys = 5,
}

impl From<SerializationError> for CErrorCode {
    fn from(value: SerializationError) -> Self {
        match value {
            SerializationError::NotEnoughSpace => CErrorCode::SerializationErrorNotEnoughSpace,
            SerializationError::InvalidData => CErrorCode::SerializationErrorInvalidData,
            SerializationError::UnexpectedFlags => CErrorCode::SerializationErrorUnexpectedFlags,
            SerializationError::IoError(_) => CErrorCode::SerializationErrorIoError,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CReturn<T> {
    pub(crate) value: T,
    pub(crate) err_code: CErrorCode,
}
