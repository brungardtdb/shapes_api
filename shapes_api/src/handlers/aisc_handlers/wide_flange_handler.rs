use crate::dto::aisc_shapes::WideFlange;
use crate::error_handling::aisc::{AISCError, AppJson};
use shapes::aisc_shapes::WideFlange as WF;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Wide Flange Handler
pub struct AppStateDyn {
    /// Repository for wide flange shapes
    pub repo: Arc<dyn ShapeRepository<WF>>,
}

/// Gets all wide flange AISC shapes
#[debug_handler]
pub async fn get_all(
    State(state): State<Arc<AppStateDyn>>,
) -> Result<AppJson<Vec<WideFlange>>, AISCError> {
    let shapes_result = &state.repo.all().await;
    match shapes_result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => {
            if shapes.iter().count() < 1 {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<WideFlange> = shapes.iter().map(|s: &WF| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}
