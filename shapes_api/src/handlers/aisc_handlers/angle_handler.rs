use crate::dto;
use crate::dto::aisc_shapes::Angle;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::Angle as A;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Angle Handler
pub struct AppStateDyn {
    /// Repository for AISC angles
    pub repo: Arc<dyn ShapeRepository<A>>,
}

#[derive(Debug, Deserialize)]
/// Query parameters for AISC wide flange
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Beam Depth
    pub long_leg_width: Option<f64>,
    /// Beam Width
    pub short_leg_width: Option<f64>,
}

/// Gets all AISC angles
#[debug_handler]
pub async fn get(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<Angle>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.iter().count() < 1 {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<Angle> = shapes.iter().map(|s| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(_err) => return Err(AISCError::ShapeNotFound),
            Ok(s) => {
                let shape: dto::aisc_shapes::Angle = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    // Check for EDI Std Nomenclature
    if let Some(nom) = params.edi_std_nomenclature.clone() {
        let shape_result = &state.repo.shape_with_edi_std_nomenclature(nom).await;
        match shape_result {
            Err(_err) => return Err(AISCError::ShapeNotFound),
            Ok(s) => {
                let shape: dto::aisc_shapes::Angle = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    // Check for long and short angle legs
    match (params.long_leg_width, params.short_leg_width) {
        (Some(long_leg), Some(short_leg)) => {
            let shapes_result = &state.repo.shapes_with_width(long_leg).await;
            match shapes_result {
                Ok(shapes) => {
                    if shapes.iter().len() == 0 {
                        return Err(AISCError::ShapeNotFound);
                    }
                    return Ok(AppJson(
                        shapes
                            .iter()
                            .filter(|s| s.d_lower == short_leg)
                            .map(|s| s.into())
                            .collect::<Vec<_>>(),
                    ));
                }
                Err(err) => {
                    return Err(AISCError::DataError(Box::from(err.to_string())));
                }
            }
        }
        (Some(long_leg), None) => {
            let shapes_result = &state.repo.shapes_with_width(long_leg).await;
            match shapes_result {
                Ok(shapes) => {
                    if shapes.iter().len() == 0 {
                        return Err(AISCError::ShapeNotFound);
                    }
                    return Ok(AppJson(shapes.iter().map(|s| s.into()).collect::<Vec<_>>()));
                }
                Err(err) => {
                    return Err(AISCError::DataError(Box::from(err.to_string())));
                }
            }
        }
        (None, Some(short_leg)) => {
            let shapes_result = &state.repo.shapes_with_depth(short_leg).await;
            match shapes_result {
                Ok(shapes) => {
                    if shapes.iter().len() == 0 {
                        return Err(AISCError::ShapeNotFound);
                    }
                    return Ok(AppJson(shapes.iter().map(|s| s.into()).collect::<Vec<_>>()));
                }
                Err(err) => {
                    return Err(AISCError::DataError(Box::from(err.to_string())));
                }
            }
        }
        _ => {
            // No angle legs specified
            return Ok(AppJson(Vec::new()));
        }
    }
}

fn has_query(params: &Params) -> bool {
    if let Some(_) = params.aisc_manual_label {
        return true;
    }
    if let Some(_) = params.edi_std_nomenclature {
        return true;
    }
    if let Some(_) = params.long_leg_width {
        return true;
    }
    if let Some(_) = params.short_leg_width {
        return true;
    }
    return false;
}
