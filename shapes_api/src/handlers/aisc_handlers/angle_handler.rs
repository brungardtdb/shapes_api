use crate::dto;
use crate::dto::aisc_shapes::Angle;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use shapes::aisc_shapes::{Angle as A};
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
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    let mut angles: Vec<Angle> = Vec::new();
    if let Some(shorter_leg) = params.short_leg_width {
        let mut a = get_from_shorter_leg(&state, shorter_leg, &mut angles).await?;
        angles.append(&mut a);
    }
    if let Some(longer_leg) = params.long_leg_width {
        let mut a = get_from_longer_leg(&state, longer_leg, &mut angles).await?;
        angles.append(&mut a);
    }
    Ok(AppJson(angles))
}

async fn get_from_shorter_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    angles: &mut Vec<Angle>,
) -> Result<Vec<Angle>, AISCError> {
    if angles.iter().nth(1).is_some() {
        return Ok(angles
            .iter()
            .filter(|a| a.d_lower == leg)
            .map(|a| a.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_depth(leg).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => {
            return Ok(shapes.iter().map(|s| s.into()).collect::<Vec<_>>());
        }
    }
}

async fn get_from_longer_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    angles: &mut Vec<Angle>,
) -> Result<Vec<Angle>, AISCError> {
    if angles.iter().nth(1).is_some() {
        return Ok(angles
            .iter()
            .filter(|a| a.b_lower == leg)
            .map(|a| a.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_width(leg).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => {
            return Ok(shapes.iter().map(|s| s.into()).collect::<Vec<_>>());
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
