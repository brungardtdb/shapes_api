use axum::{
    extract::FromRequest,
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
    ShapeNotFound,
}

impl std::fmt::Display for AISCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AISCError::DataError(err) => write!(f, "{err}"),
            AISCError::ShapeError(err) => write!(f, "{err}"),
            AISCError::ShapeNotFound => write!(f, "Could not find the requested shape"),
        }
    }
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
            AISCError::ShapeNotFound => (
                StatusCode::NOT_FOUND,
                "Could not find the requested shape".to_owned(),
                None,
            ),
        };

        let mut response = (status, AppJson(ErrorResponse { message })).into_response();
        if let Some(err) = err {
            response.extensions_mut().insert(Arc::new(err));
        }
        response
    }
}

// TODO - SET UP MIDDLEWARE TO LOG APPLICATION ERRORS?
