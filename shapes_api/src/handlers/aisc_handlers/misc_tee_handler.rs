use crate::dto;
use crate::dto::aisc_shapes::MiscTee;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::MiscTee as MT;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for Misc Tee Handler
pub struct AppStateDyn {
    /// Repository for misc tee shapes
    pub repo: Arc<dyn ShapeRepository<MT>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC misc tee
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Beam Depth
    pub detailing_depth: Option<f64>,
    /// Beam Width
    pub detailing_flange_width: Option<f64>,
}

/// Gets all misc tee AISC shapes
#[debug_handler]
pub async fn get(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<MiscTee>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<MiscTee>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => {
            if shapes.is_empty() {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<MiscTee> = shapes.iter().map(|s: &MT| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<MiscTee>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::MiscTee = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    // Check for EDI Std Nomenclature
    if let Some(nom) = params.edi_std_nomenclature.clone() {
        let shape_result = &state.repo.shape_with_edi_std_nomenclature(nom).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::MiscTee = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<MiscTee>>, AISCError> {
    let mut tees: Vec<MiscTee> = Vec::new();
    if let Some(depth) = params.detailing_depth {
        tees = get_from_detailing_depth(&state, depth, &mut tees).await?;
    }
    if let Some(flange_width) = params.detailing_flange_width {
        tees = get_from_detailing_flange_width(&state, flange_width, &mut tees).await?;
    }
    Ok(AppJson(tees))
}

async fn get_from_detailing_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    tees: &mut Vec<MiscTee>,
) -> Result<Vec<MiscTee>, AISCError> {
    if tees.iter().nth(0).is_some() {
        return Ok(tees
            .iter()
            .filter(|t| t.ddet == depth)
            .map(|t| t.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_depth(depth).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => return Ok(shapes.iter().map(|s| s.into()).collect::<Vec<_>>()),
    }
}

async fn get_from_detailing_flange_width(
    state: &Arc<AppStateDyn>,
    flange_width: f64,
    tees: &mut Vec<MiscTee>,
) -> Result<Vec<MiscTee>, AISCError> {
    if tees.iter().nth(0).is_some() {
        return Ok(tees
            .iter()
            .filter(|t| t.bfdet == flange_width)
            .map(|t| t.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_width(flange_width).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => return Ok(shapes.iter().map(|s| s.into()).collect::<Vec<_>>()),
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
