use crate::dto;
use crate::dto::aisc_shapes::HollowStructuralSection;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::HollowStructuralSection as HSS;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Hollow Structural Section Handler
pub struct AppStateDyn {
    /// Repository for HSS shapes
    pub repo: Arc<dyn ShapeRepository<HSS>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC hollow structural sections
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Member Depth (Ht)
    pub member_depth: Option<f64>,
    /// Member Width (B)
    pub member_width: Option<f64>,
}

/// Gets all hollow structural section AISC shapes
#[debug_handler]
pub async fn get_hollow_structural_sections(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<HollowStructuralSection>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(
    state: Arc<AppStateDyn>,
) -> Result<AppJson<Vec<HollowStructuralSection>>, AISCError> {
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

            let result: Vec<HollowStructuralSection> =
                shapes.iter().map(|s: &HSS| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<HollowStructuralSection>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::HollowStructuralSection = s.into();
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
                let shape: dto::aisc_shapes::HollowStructuralSection = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<HollowStructuralSection>>, AISCError> {
    let mut sections: Vec<HollowStructuralSection> = Vec::new();
    if let Some(depth) = params.member_depth {
        sections = get_from_member_depth(&state, depth, &mut sections).await?;
    }
    if let Some(width) = params.member_width {
        sections = get_from_member_width(&state, width, &mut sections).await?;
    }
    Ok(AppJson(sections))
}

async fn get_from_member_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    sections: &mut Vec<HollowStructuralSection>,
) -> Result<Vec<HollowStructuralSection>, AISCError> {
    if sections.iter().nth(0).is_some() {
        return Ok(sections
            .iter()
            .filter(|s| s.ht == depth)
            .map(|s| s.clone())
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

async fn get_from_member_width(
    state: &Arc<AppStateDyn>,
    width: f64,
    sections: &mut Vec<HollowStructuralSection>,
) -> Result<Vec<HollowStructuralSection>, AISCError> {
    if sections.iter().nth(0).is_some() {
        return Ok(sections
            .iter()
            .filter(|s| s.b_upper == width)
            .map(|s| s.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_width(width).await;
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
    if let Some(_) = params.member_depth {
        return true;
    }
    if let Some(_) = params.member_width {
        return true;
    }
    return false;
}
