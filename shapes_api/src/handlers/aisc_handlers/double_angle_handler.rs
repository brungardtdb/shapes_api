use crate::dto;
use crate::dto::aisc_shapes::DoubleAngle;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::DoubleAngle as DE;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Double Angle Handler
pub struct AppStateDyn {
    /// Repository for AISC cee channels
    pub repo: Arc<dyn ShapeRepository<DE>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC double angles
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

/// Gets all AISC double angles
#[debug_handler]
pub async fn get(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    let result = state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.iter().count() < 1 {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<DoubleAngle> = shapes.iter().map(|d| d.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => {
                // TODO - THIS SHOULD PROBABLY NOT LIVE IN THE HANDLER,
                // BUT WE NEED TO KNOW IF THE DB FAILED OR IF THE QUERY RETURNED NO RESULTS
                if *&err.to_string().starts_with("no rows returned") {
                    return Err(AISCError::ShapeNotFound);
                } else {
                    return Err(AISCError::DataError(Box::from(err.to_string())));
                }
            }
            Ok(d) => {
                let shape: dto::aisc_shapes::DoubleAngle = d.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }

    if let Some(std_nom) = params.edi_std_nomenclature.clone() {
        let shape_result = &state.repo.shape_with_edi_std_nomenclature(std_nom).await;
        match shape_result {
            Err(err) => {
                // TODO - THIS SHOULD PROBABLY NOT LIVE IN THE HANDLER,
                // BUT WE NEED TO KNOW IF THE DB FAILED OR IF THE QUERY RETURNED NO RESULTS
                if *&err.to_string().starts_with("no rows returned") {
                    return Err(AISCError::ShapeNotFound);
                } else {
                    return Err(AISCError::DataError(Box::from(err.to_string())));
                }
            }
            Ok(d) => {
                let shape: dto::aisc_shapes::DoubleAngle = d.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }

    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    let mut double_angles: Vec<DoubleAngle> = Vec::new();
    if let Some(shorter_leg) = params.short_leg_width {
        double_angles = get_from_shorter_leg(&state, shorter_leg, &mut double_angles).await?;
    }
    if let Some(longer_leg) = params.long_leg_width {
        double_angles = get_from_longer_leg(&state, longer_leg, &mut double_angles).await?;
    }
    Ok(AppJson(double_angles))
}

async fn get_from_shorter_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    double_angles: &mut Vec<DoubleAngle>,
) -> Result<Vec<DoubleAngle>, AISCError> {
    if double_angles.iter().nth(1).is_some() {
        return Ok(double_angles
            .iter()
            .filter(|d| d.d_lower == leg)
            .map(|d| d.clone())
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
    double_angles: &mut Vec<DoubleAngle>,
) -> Result<Vec<DoubleAngle>, AISCError> {
    if double_angles.iter().nth(1).is_some() {
        return Ok(double_angles
            .iter()
            .filter(|d| d.b_lower == leg)
            .map(|d| d.clone())
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
