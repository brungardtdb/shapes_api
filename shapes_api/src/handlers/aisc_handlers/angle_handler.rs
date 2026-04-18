use crate::dto::aisc_shapes::Angle;
use crate::error_handling::aisc::{AISCError, AppJson};
use shapes::aisc_shapes::Angle as A;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Angle Handler
pub struct AppStateDyn {
    /// Repository for AISC angles
    pub repo: Arc<dyn ShapeRepository<A>>,
}

/// Gets all AISC angles
#[debug_handler]
pub async fn get_all(
    State(state): State<Arc<AppStateDyn>>
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())))
        },
        Ok(shapes) => {
            if shapes.iter().count() < 1 {
                return Err(AISCError::DataError(Box::from("Unable to retrieve shapes from the AISC shape database".to_owned())))
            }

            let result: Vec<Angle> = shapes.iter().map(|s| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}