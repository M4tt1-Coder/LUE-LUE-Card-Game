use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {

use crate::backend::errors::application_error::{ApplicationError, ErrorObject};

use axum::{http::StatusCode, response::IntoResponse, Json};

/// This module defines a custom error type for handling database query errors.
///
/// It provides a structure to encapsulate the error message related to database operations,
/// allowing for better error handling and reporting in applications that interact with a database.
///
/// # Example Usage
///
/// ```rust
/// use crate::types::database_query_error::DatabaseQueryError;
///
/// fn perform_database_query() -> Result<(), DatabaseQueryError> {
///     // Simulate a database query operation
///     let success = false; // Simulating a failure
///     if !success {
///     Err(DatabaseQueryError {
///         message: "Failed to execute database query".to_string(),
///     })
///     } else {
///     Ok(())
///     }
/// }
/// ```
///
/// # Properties
///
/// - `message`: A string that contains the error message describing the database query error.
pub struct DatabaseQueryError<T: for<'a> ErrorObject<'a>> {
    /// The error message describing the database query error.
    pub message: String,
    /// The data received from the database that caused the error, if applicable.
    pub received_data: Option<Json<T>>,
    /// The HTTP status code associated with the error.
    pub status_code: StatusCode,
}

// ----- Implementation 'DatabaseQueryError' -----

impl<T: for<'a> ErrorObject<'a>> DatabaseQueryError<T> {
    /// Creates a new `DatabaseQueryError` instance with the provided error message.
    ///
    /// # Arguments
    ///
    /// - `message`: A string that describes the error encountered during the database query.
    /// - `received_data`: An optional JSON object containing the data received from the database
    ///
    /// # Returns
    ///
    /// A new `DatabaseQueryError` instance.
    pub fn new(message: String, received_data: Option<Json<T>>, status_code: StatusCode) -> Self {
        DatabaseQueryError {
            message,
            received_data,
            status_code,
        }
    }
}

// ----- Implementation 'ApplicationError' for 'DatabaseQueryError' -----

impl<T: for<'a> ErrorObject<'a>> std::fmt::Display for DatabaseQueryError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Database query error: {}. Received data: {:?}",
            self.message, self.received_data
        )
    }
}

impl<T: for<'a> ErrorObject<'a>> std::fmt::Debug for DatabaseQueryError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "DatabaseQueryError {{ message: {}, received_data: {:?} }}",
            self.message, self.received_data
        )
    }
}

impl<T: for<'a> ErrorObject<'a>> std::error::Error for DatabaseQueryError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }
}

impl<T: for<'a> ErrorObject<'a>> ApplicationError for DatabaseQueryError<T> {}

// ----- Implementation of the Axum 'IntoResponse' trait for the 'DatabaseQueryError<T>' struct

impl<T: for<'a> ErrorObject<'a>> IntoResponse for DatabaseQueryError<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, self).into_response()
    }
}

    }
}
//     }
