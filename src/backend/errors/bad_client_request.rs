use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

        use std::fmt::{self, Debug, Display};
        use axum::{http::StatusCode, response::IntoResponse, Json};
        use crate::backend::errors::application_error::{ApplicationError, ErrorObject};

/// Error type for all request with invalid data a client sends to the backend.
///
/// # Props
///
/// - 'message': Description what error occured.
/// - 'bad_data': The data object of type 'T' that caused the error.
pub struct BadClientRequest<T: for<'a> ErrorObject<'a>> {
    /// Message for the client what he / she did wrong
    pub message: String,
    /// Data object as origin for the error
    pub bad_data: Json<T>,
}

impl<T: for<'a> ErrorObject<'a>> Display for BadClientRequest<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bad request was sent by a client! Error: {} ... caused by: {:?}!",
            self.message, self.bad_data
        )
    }
}

impl<T: for<'a> ErrorObject<'a>> Debug for BadClientRequest<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bad request was sent by a client! Error: {} ... caused by: {:?}!",
            self.message, self.bad_data
        )
    }
}

impl<T: for<'a> ErrorObject<'a>> std::error::Error for BadClientRequest<T> {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<T: for<'a> ErrorObject<'a>> BadClientRequest<T> {
    /// Resembling http status code for a bad request
    pub const STATUS_CODE: StatusCode = StatusCode::BAD_REQUEST;

    /// Creates a new 'BadClientRequest' instance with a indiviual error message.
    ///
    /// # Params
    ///
    /// - 'message': Error message
    /// - 'bad_data': Request body or wrongly created data object
    ///
    /// # Returns
    ///
    /// BadClientRequest<'a, T>: Specific error for different endpoints for example.
    ///
    pub fn new(message: String, bad_data: Json<T>) -> BadClientRequest<T> {
        BadClientRequest { message, bad_data }
    }
}

// ----- Implementation of the Axum trait 'IntoResponse' for the 'BadClientRequest' struct -----

impl<T: for<'a> ErrorObject<'a>> IntoResponse for BadClientRequest<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, self).into_response()
    }
}

// impl<T: for<'a> ErrorObject<'a>> IntoResponseParts for BadClientRequest<T> {
//    type Error = ProcessError<T>;

//    fn into_response_parts(
//        self,
//        res: axum::response::ResponseParts,
//    ) -> Result<axum::response::ResponseParts, Self::Error> {
//    }
// }

// ----- Implementation of the 'ApplicationError' trait for the 'BadClientRequest' struct -----

impl<T: for<'a> ErrorObject<'a>> ApplicationError for BadClientRequest<T> {}
    }
}
