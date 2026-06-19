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
    /// Repository for AISC cee channels
    pub repo: Arc<dyn ShapeRepository<Cee>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
pub async fn get_cee_channels(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.is_empty() {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<CeeChannel> = shapes.iter().map(|s| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::CeeChannel = s.into();
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
                let shape: dto::aisc_shapes::CeeChannel = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<CeeChannel>>, AISCError> {
    let mut channels: Vec<CeeChannel> = Vec::new();
    if let Some(depth) = params.detailing_depth {
        channels = get_from_detailing_depth(&state, depth, &mut channels).await?;
    }
    if let Some(flange_width) = params.detailing_flange_width {
        channels = get_from_detailing_flange_width(&state, flange_width, &mut channels).await?;
    }
    Ok(AppJson(channels))
}

async fn get_from_detailing_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    channels: &mut Vec<CeeChannel>,
) -> Result<Vec<CeeChannel>, AISCError> {
    if channels.iter().nth(0).is_some() {
        return Ok(channels
            .iter()
            .filter(|c| c.ddet == depth)
            .map(|c| c.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_depth(depth).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(channels) => {
            return Ok(channels.iter().map(|c| c.into()).collect::<Vec<_>>());
        }
    }
}

async fn get_from_detailing_flange_width(
    state: &Arc<AppStateDyn>,
    flange_width: f64,
    channels: &mut Vec<CeeChannel>,
) -> Result<Vec<CeeChannel>, AISCError> {
    if channels.iter().nth(0).is_some() {
        return Ok(channels
            .iter()
            .filter(|c| c.bfdet == flange_width)
            .map(|c| c.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_width(flange_width).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(channels) => {
            return Ok(channels.iter().map(|c| c.into()).collect::<Vec<_>>());
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

#[cfg(test)]
#[doc(hidden)]
pub mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use mockall::{mock, predicate};
    use shapes::aisc_shapes::shape_builder::ShapeBuilder;
    use shapes::aisc_shapes::shape_repository::ShapeRepository;
    use shapes::aisc_shapes::{CeeChannel as C, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub ChannelRepo {}

        impl ShapeRepository<C> for ChannelRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<C>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String
            ) -> Pin<Box<dyn Future<Output = Result<Option<C>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String
            ) -> Pin<Box<dyn Future<Output = Result<Option<C>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64
            ) -> Pin<Box<dyn Future<Output = Result<Vec<C>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64
            ) -> Pin<Box<dyn Future<Output = Result<Vec<C>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockChannelRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async {
                Ok(vec![
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("C8X13.75"))
                        .with_aisc_manual_label(String::from("C8X13.75"))
                        .with_w_upper(13.75)
                        .with_a_upper(4.03)
                        .with_d_lower(8.0)
                        .with_ddet(8.0)
                        .with_bf(2.34)
                        .with_bfdet(2.375)
                        .with_tw(0.303)
                        .with_twdet(0.3125)
                        .with_twdet_2(0.1875)
                        .with_tf(0.390)
                        .with_tfdet(0.375)
                        .with_kdes(0.938)
                        .with_kdet(0.9375)
                        .with_x_lower(0.554)
                        .with_eo(0.604)
                        .with_xp(0.252)
                        .with_b_t(6.0)
                        .with_h_tw(21.0)
                        .with_ix(36.1)
                        .with_zx(11.0)
                        .with_sx(9.02)
                        .with_rx(2.99)
                        .with_iy(1.52)
                        .with_zy(1.73)
                        .with_sy(0.848)
                        .with_ry(0.613)
                        .with_j_upper(0.186)
                        .with_cw(19.2)
                        .with_wno(5.45)
                        .with_sw1(1.52)
                        .with_sw2(1.10)
                        .with_sw3(0.557)
                        .with_qf(3.02)
                        .with_qw(5.45)
                        .with_ro(3.25)
                        .with_h_upper(0.874)
                        .with_rts(0.774)
                        .with_ho(7.61)
                        .with_pa(22.1)
                        .with_pb(24.4)
                        .with_pc(18.3)
                        .with_pd(20.7)
                        .with_t(6.125)
                        .with_wgi(1.375)
                        .try_build::<C>()
                        .unwrap(),
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("C8X11.5"))
                        .with_aisc_manual_label(String::from("C8X11.5"))
                        .with_w_upper(11.5)
                        .with_a_upper(3.37)
                        .with_d_lower(8.0)
                        .with_ddet(8.0)
                        .with_bf(2.26)
                        .with_bfdet(2.25)
                        .with_tw(0.22)
                        .with_twdet(0.25)
                        .with_twdet_2(0.125)
                        .with_tf(0.390)
                        .with_tfdet(0.375)
                        .with_kdes(0.938)
                        .with_kdet(0.9375)
                        .with_x_lower(0.572)
                        .with_eo(0.697)
                        .with_xp(0.211)
                        .with_b_t(5.79)
                        .with_h_tw(28.9)
                        .with_ix(32.5)
                        .with_zx(9.63)
                        .with_sx(8.14)
                        .with_rx(3.11)
                        .with_iy(1.31)
                        .with_zy(1.57)
                        .with_sy(0.775)
                        .with_ry(0.623)
                        .with_j_upper(0.13)
                        .with_cw(16.5)
                        .with_wno(5.11)
                        .with_sw1(1.34)
                        .with_sw2(0.855)
                        .with_sw3(0.430)
                        .with_qf(3.03)
                        .with_qw(4.79)
                        .with_ro(3.41)
                        .with_h_upper(0.862)
                        .with_rts(0.756)
                        .with_ho(7.61)
                        .with_pa(21.9)
                        .with_pb(24.1)
                        .with_pc(18.3)
                        .with_pd(20.5)
                        .with_t(6.125)
                        .with_wgi(1.375)
                        .try_build::<C>()
                        .unwrap(),
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("C7X14.75"))
                        .with_aisc_manual_label(String::from("C7X14.75"))
                        .with_w_upper(14.75)
                        .with_a_upper(4.33)
                        .with_d_lower(7.0)
                        .with_ddet(7.0)
                        .with_bf(2.3)
                        .with_bfdet(2.25)
                        .with_tw(0.419)
                        .with_twdet(0.4375)
                        .with_twdet_2(0.25)
                        .with_tf(0.366)
                        .with_tfdet(0.375)
                        .with_kdes(0.875)
                        .with_kdet(0.875)
                        .with_x_lower(0.532)
                        .with_eo(0.441)
                        .with_xp(0.309)
                        .with_b_t(6.28)
                        .with_h_tw(12.9)
                        .with_ix(27.2)
                        .with_zx(9.75)
                        .with_sx(7.78)
                        .with_rx(2.51)
                        .with_iy(1.37)
                        .with_zy(1.63)
                        .with_sy(0.772)
                        .with_ry(0.561)
                        .with_j_upper(0.267)
                        .with_cw(13.1)
                        .with_wno(4.78)
                        .with_sw1(1.26)
                        .with_sw2(1.0)
                        .with_sw3(0.498)
                        .with_qf(2.28)
                        .with_qw(4.85)
                        .with_ro(2.75)
                        .with_h_upper(0.875)
                        .with_rts(0.738)
                        .with_ho(6.63)
                        .with_pa(20.0)
                        .with_pb(22.3)
                        .with_pc(16.3)
                        .with_pd(18.6)
                        .with_t(5.25)
                        .with_wgi(1.25)
                        .try_build::<C>()
                        .unwrap(),
                ])
            })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/channels", get(get_cee_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/channels").await;

        response.assert_status_ok();
        let channels: Vec<CeeChannel> = response.json::<Vec<CeeChannel>>();
        assert_eq!(3, channels.iter().count());
    }
}
