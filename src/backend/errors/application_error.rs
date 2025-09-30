use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // TODO: Rework all methods / functions to return a error object that implements the 'ApplicationError' trait
        use std::{error, fmt};

        use axum::response::IntoResponse;
        use serde::Deserialize;

        /// Global error trait that is implement by custom error types
        ///
        /// It brings all necessary traits that a Rust error struct needs to implement.
        ///
        /// Specific adjustments are made at all indiviual definition.
        pub trait ApplicationError: fmt::Display + error::Error + fmt::Debug + IntoResponse {}

        /// Error object trait for data types that should be logged in the console or in the error message.
        ///
        /// In some error types the causing object is inbetted in the error message.
        pub trait ErrorObject<'a>: Deserialize<'a> + fmt::Display + fmt::Debug {}
    }
}
