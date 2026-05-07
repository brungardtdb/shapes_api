use crate::dto;
use crate::dto::aisc_shapes::CeeChannel;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::CeeChannel as Cee;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Cee Channel Handler
pub struct AppStateDyn {
    /// Repository for AISC angles
    pub repo: Arc<dyn ShapeRepository<Cee>>,
}

#[derive(Debug, Deserialize)]
/// Query parameters for AISC cee channel
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Channel Depth
    pub detailing_depth: Option<f64>,
    /// Channel Flange Width
    pub detailing_flange_width: Option<f64>,
}

/// Gets all AISC Cee Channels
#[debug_handler]
pub async fn get(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(_err) => return Err(AISCError::ShapeNotFound),
            Ok(s) => {
                let shape: dto::aisc_shapes::CeeChannel = s.into();
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
                let shape: dto::aisc_shapes::CeeChannel = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    // Check for width and height
    match (params.detailing_flange_width, params.detailing_depth) {
        (Some(width), Some(depth)) => {
            let shapes_result = &state.repo.shapes_with_width(width).await;
            match shapes_result {
                Ok(shapes) => {
                    if shapes.iter().len() == 0 {
                        return Err(AISCError::ShapeNotFound);
                    }
                    return Ok(AppJson(
                        shapes
                            .iter()
                            .filter(|s| s.d_lower == depth)
                            .map(|s| s.into())
                            .collect::<Vec<_>>(),
                    ));
                }
                Err(err) => {
                    return Err(AISCError::DataError(Box::from(err.to_string())));
                }
            }
        }
        (Some(width), None) => {
            let shapes_result = &state.repo.shapes_with_width(width).await;
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
        (None, Some(depth)) => {
            let shapes_result = &state.repo.shapes_with_depth(depth).await;
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
            // No depth or width specified
            return Ok(AppJson(Vec::new()));
        }
    }
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.iter().count() < 1 {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<CeeChannel> = shapes.iter().map(|s| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
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
    if let Some(_) = params.detailing_depth {
        return true;
    }
    if let Some(_) = params.detailing_flange_width {
        return true;
    }
    return false;
}
