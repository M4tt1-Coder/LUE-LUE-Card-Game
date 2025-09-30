use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
use std::{error::Error, fmt::Display};

use axum::{http, response::IntoResponse};

use crate::backend::errors::application_error::{ApplicationError, ErrorObject};

#[derive()]
/// ## Error Struct
///
/// Occurs during interal operation like calculating something or changing properties of current
/// variables.
///
/// # Fields
///
/// - `message` -> Description of the situation
/// - `name_of_function` -> Name of the function / method where the error occured
/// - `bad_data` -> some data that could have played a critical role in the situation
///
/// # Example
///
/// ```rust
/// fn some_func() -> Result<(), ProcessObject<SomeData>> {
///     return Err(SomeData::new());
/// }
/// ```
#[derive(Debug)]
pub struct ProcessError<T: for<'a> ErrorObject<'a>> {
    /// Descriptive text which indicates the problem and explains it.
    pub message: String,
    /// Place where the problem occured -> Name of the function itself
    pub name_of_function: String,
    /// Optional data that maybe caused the issue or was / is part of it
    pub bad_data: Option<T>,
}

impl<T: for<'a> ErrorObject<'a>> ProcessError<T> {
    /// Creates and returns a new instance of the `ProcessError` struct.
    ///
    /// # Example
    ///
    /// ```rust
    ///     let err = ProcessError::new("A message".to_string(), "this_func".to_string(), None)
    /// ```
    pub fn new(message: String, name_of_function: String, bad_data: Option<T>) -> Self {
        ProcessError {
            message,
            name_of_function,
            bad_data,
        }
    }
}

// ----- Implementation of 'ApplicationError' trait for 'ProcessError' struct -----

impl<T: for<'a> ErrorObject<'a>> Display for ProcessError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Message: {}, Name of the Function: {}, Possible invalid Data: {:?}",
            self.message, self.name_of_function, self.bad_data
        )
    }
}

impl<T: for<'a> ErrorObject<'a>> Error for ProcessError<T> {}

impl<T: for<'a> ErrorObject<'a>> ApplicationError for ProcessError<T> {}

// ----- Implementation of the Axum 'IntoResponse' trait for the 'ProcessError<T>' struct -----

impl<T: for<'a> ErrorObject<'a>> IntoResponse for ProcessError<T> {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::OK, self).into_response()
    }
}

    }
}
