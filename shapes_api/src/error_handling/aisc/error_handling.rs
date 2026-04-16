use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use shapes::aisc_shapes::errors::MissingPropertyError;
use std::error::Error;
use std::sync::Arc;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AISCError))]
/// Wrapper for axum JSON, makes overriding default responses easier// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug)]
/// Possible errors that can be returned by the AISC shapes API
pub enum AISCError {
    /// Missing property when constructing AISC shape
    ShapeError(MissingPropertyError),
    /// Unexpected error
    DataError(Box<dyn Error + Send + Sync>),
    /// Shape not found
    ShapeNotFound(JsonRejection),
}

impl IntoResponse for AISCError {
    fn into_response(self) -> Response {
        /// Serialize errors into a response
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message, err) = match &self {
            AISCError::DataError(_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "The server encountered an unexpected problem".to_owned(),
                Some(self),
            ),
            AISCError::ShapeError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
                Some(self),
            ),
            AISCError::ShapeNotFound(rejection) => {
                (StatusCode::NOT_FOUND, rejection.to_string(), None)
            }
        };

        let mut response = (status, AppJson(ErrorResponse { message })).into_response();
        if let Some(err) = err {
            response.extensions_mut().insert(Arc::new(err));
        }
        response
    }
}

impl From<JsonRejection> for AISCError {
    fn from(rejection: JsonRejection) -> Self {
        Self::ShapeNotFound(rejection)
    }
}

// TODO - SET UP MIDDLEWARE TO LOG APPLICATION ERRORS?
