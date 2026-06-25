use crate::dto;
use crate::dto::aisc_shapes::HPile;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::HPile as HP;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for H-Pile Handler
pub struct AppStateDyn {
    /// Repository for h-pile shapes
    pub repo: Arc<dyn ShapeRepository<HP>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC h-pile
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

/// Gets all h-pile AISC shapes
#[debug_handler]
pub async fn get_h_piles(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<HPile>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<HPile>>, AISCError> {
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

            let result: Vec<HPile> = shapes.iter().map(|s: &HP| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<HPile>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::HPile = s.into();
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
                let shape: dto::aisc_shapes::HPile = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<HPile>>, AISCError> {
    let mut piles: Vec<HPile> = Vec::new();
    if let Some(depth) = params.detailing_depth {
        piles = get_from_detailing_depth(&state, depth, &mut piles).await?;
        if piles.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(flange_width) = params.detailing_flange_width {
        piles = get_from_detailing_flange_width(&state, flange_width, &mut piles).await?;
        if piles.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    Ok(AppJson(piles))
}

async fn get_from_detailing_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    piles: &mut Vec<HPile>,
) -> Result<Vec<HPile>, AISCError> {
    if piles.iter().nth(0).is_some() {
        return Ok(piles
            .iter()
            .filter(|p| p.ddet == depth)
            .map(|p| p.clone())
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
    piles: &mut Vec<HPile>,
) -> Result<Vec<HPile>, AISCError> {
    if piles.iter().nth(0).is_some() {
        return Ok(piles
            .iter()
            .filter(|p| p.bfdet == flange_width)
            .map(|p| p.clone())
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

#[cfg(test)]
#[doc(hidden)]
pub mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use mockall::{mock, predicate};
    use shapes::aisc_shapes::shape_builder::ShapeBuilder;
    use shapes::aisc_shapes::shape_repository::ShapeRepository;
    use shapes::aisc_shapes::{HPile as HP, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub HPileRepo {}

        impl ShapeRepository<HP> for HPileRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<HP>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<HP>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<HP>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<HP>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<HP>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockHPileRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async {
                Ok(vec![
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("HP18X204"))
                        .with_aisc_manual_label(String::from("HP18X204"))
                        .with_w_upper(204.0)
                        .with_a_upper(60.2)
                        .with_d_lower(18.3)
                        .with_ddet(18.25)
                        .with_bf(18.1)
                        .with_bfdet(18.125)
                        .with_tw(1.13)
                        .with_twdet(1.125)
                        .with_twdet_2(0.5625)
                        .with_tf(1.13)
                        .with_tfdet(1.125)
                        .with_kdes(2.31)
                        .with_kdet(2.3125)
                        .with_k1(1.75)
                        .with_bf_2tf(8.01)
                        .with_h_tw(12.1)
                        .with_ix(3480.0)
                        .with_zx(433.0)
                        .with_sx(380.0)
                        .with_rx(7.6)
                        .with_iy(1120.0)
                        .with_zy(191.0)
                        .with_sy(124.0)
                        .with_ry(4.31)
                        .with_j_upper(29.5)
                        .with_cw(82500.0)
                        .with_wno(77.7)
                        .with_sw1(397.0)
                        .with_qf(82.3)
                        .with_qw(212.0)
                        .with_rts(5.03)
                        .with_ho(17.2)
                        .with_pa(86.6)
                        .with_pb(105.0)
                        .with_pc(54.7)
                        .with_pd(72.8)
                        .with_t(13.5)
                        .with_wgi(7.5)
                        .try_build::<HP>()
                        .unwrap(),
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("HP18X181"))
                        .with_aisc_manual_label(String::from("HP18X181"))
                        .with_w_upper(181.0)
                        .with_a_upper(53.2)
                        .with_d_lower(18.0)
                        .with_ddet(18.0)
                        .with_bf(18.0)
                        .with_bfdet(18.0)
                        .with_tw(1.0)
                        .with_twdet(1.0)
                        .with_twdet_2(0.5)
                        .with_tf(1.0)
                        .with_tfdet(1.0)
                        .with_kdes(2.18)
                        .with_kdet(2.1875)
                        .with_k1(1.6875)
                        .with_bf_2tf(9.0)
                        .with_h_tw(13.6)
                        .with_ix(3020.0)
                        .with_zx(379.0)
                        .with_sx(336.0)
                        .with_rx(7.53)
                        .with_iy(974.0)
                        .with_zy(167.0)
                        .with_sy(108.0)
                        .with_ry(4.28)
                        .with_j_upper(20.7)
                        .with_cw(70400.0)
                        .with_wno(76.5)
                        .with_sw1(344.0)
                        .with_qf(72.3)
                        .with_qw(185.0)
                        .with_rts(4.96)
                        .with_ho(17.0)
                        .with_pa(86.0)
                        .with_pb(104.0)
                        .with_pc(54.0)
                        .with_pd(72.0)
                        .with_t(13.5)
                        .with_wgi(7.5)
                        .try_build::<HP>()
                        .unwrap(),
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("HP16X183"))
                        .with_aisc_manual_label(String::from("HP16X183"))
                        .with_w_upper(183.0)
                        .with_a_upper(54.1)
                        .with_d_lower(16.5)
                        .with_ddet(16.5)
                        .with_bf(16.3)
                        .with_bfdet(16.5)
                        .with_tw(1.13)
                        .with_twdet(1.125)
                        .with_twdet_2(0.5625)
                        .with_tf(1.13)
                        .with_tfdet(1.125)
                        .with_kdes(2.31)
                        .with_kdet(2.3125)
                        .with_k1(1.75)
                        .with_bf_2tf(7.21)
                        .with_h_tw(10.5)
                        .with_ix(2510.0)
                        .with_zx(349.0)
                        .with_sx(304.0)
                        .with_rx(6.81)
                        .with_iy(818.0)
                        .with_zy(156.0)
                        .with_sy(100.0)
                        .with_ry(3.89)
                        .with_j_upper(26.9)
                        .with_cw(48300.0)
                        .with_wno(62.2)
                        .with_sw1(285.0)
                        .with_qf(65.4)
                        .with_qw(169.0)
                        .with_rts(4.55)
                        .with_ho(15.4)
                        .with_pa(77.6)
                        .with_pb(93.9)
                        .with_pc(49.3)
                        .with_pd(65.6)
                        .with_t(11.75)
                        .with_wgi(5.5)
                        .try_build::<HP>()
                        .unwrap(),
                ])
            })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles").await;

        response.assert_status_ok();
        let piles: Vec<HPile> = response.json::<Vec<HPile>>();
        assert_eq!(3, piles.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HP18X204")))
            .returning(|_| {
                Box::pin(async {
                    Ok(Some(
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X204"))
                            .with_aisc_manual_label(String::from("HP18X204"))
                            .with_w_upper(204.0)
                            .with_a_upper(60.2)
                            .with_d_lower(18.3)
                            .with_ddet(18.25)
                            .with_bf(18.1)
                            .with_bfdet(18.125)
                            .with_tw(1.13)
                            .with_twdet(1.125)
                            .with_twdet_2(0.5625)
                            .with_tf(1.13)
                            .with_tfdet(1.125)
                            .with_kdes(2.31)
                            .with_kdet(2.3125)
                            .with_k1(1.75)
                            .with_bf_2tf(8.01)
                            .with_h_tw(12.1)
                            .with_ix(3480.0)
                            .with_zx(433.0)
                            .with_sx(380.0)
                            .with_rx(7.6)
                            .with_iy(1120.0)
                            .with_zy(191.0)
                            .with_sy(124.0)
                            .with_ry(4.31)
                            .with_j_upper(29.5)
                            .with_cw(82500.0)
                            .with_wno(77.7)
                            .with_sw1(397.0)
                            .with_qf(82.3)
                            .with_qw(212.0)
                            .with_rts(5.03)
                            .with_ho(17.2)
                            .with_pa(86.6)
                            .with_pb(105.0)
                            .with_pc(54.7)
                            .with_pd(72.8)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                    ))
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?aisc_manual_label=HP18X204").await;

        response.assert_status_ok();
        let piles: Vec<HPile> = response.json::<Vec<HPile>>();
        assert_eq!(1, piles.iter().count());
        assert_eq!(
            String::from("HP18X204"),
            piles.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HP18X204")))
            .returning(|_| {
                Box::pin(async {
                    Ok(Some(
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X204"))
                            .with_aisc_manual_label(String::from("HP18X204"))
                            .with_w_upper(204.0)
                            .with_a_upper(60.2)
                            .with_d_lower(18.3)
                            .with_ddet(18.25)
                            .with_bf(18.1)
                            .with_bfdet(18.125)
                            .with_tw(1.13)
                            .with_twdet(1.125)
                            .with_twdet_2(0.5625)
                            .with_tf(1.13)
                            .with_tfdet(1.125)
                            .with_kdes(2.31)
                            .with_kdet(2.3125)
                            .with_k1(1.75)
                            .with_bf_2tf(8.01)
                            .with_h_tw(12.1)
                            .with_ix(3480.0)
                            .with_zx(433.0)
                            .with_sx(380.0)
                            .with_rx(7.6)
                            .with_iy(1120.0)
                            .with_zy(191.0)
                            .with_sy(124.0)
                            .with_ry(4.31)
                            .with_j_upper(29.5)
                            .with_cw(82500.0)
                            .with_wno(77.7)
                            .with_sw1(397.0)
                            .with_qf(82.3)
                            .with_qw(212.0)
                            .with_rts(5.03)
                            .with_ho(17.2)
                            .with_pa(86.6)
                            .with_pb(105.0)
                            .with_pc(54.7)
                            .with_pd(72.8)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                    ))
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?edi_std_nomenclature=HP18X204").await;

        response.assert_status_ok();
        let piles: Vec<HPile> = response.json::<Vec<HPile>>();
        assert_eq!(1, piles.iter().count());
        assert_eq!(
            String::from("HP18X204"),
            piles.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(18.25))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X204"))
                            .with_aisc_manual_label(String::from("HP18X204"))
                            .with_w_upper(204.0)
                            .with_a_upper(60.2)
                            .with_d_lower(18.3)
                            .with_ddet(18.25)
                            .with_bf(18.1)
                            .with_bfdet(18.125)
                            .with_tw(1.13)
                            .with_twdet(1.125)
                            .with_twdet_2(0.5625)
                            .with_tf(1.13)
                            .with_tfdet(1.125)
                            .with_kdes(2.31)
                            .with_kdet(2.3125)
                            .with_k1(1.75)
                            .with_bf_2tf(8.01)
                            .with_h_tw(12.1)
                            .with_ix(3480.0)
                            .with_zx(433.0)
                            .with_sx(380.0)
                            .with_rx(7.6)
                            .with_iy(1120.0)
                            .with_zy(191.0)
                            .with_sy(124.0)
                            .with_ry(4.31)
                            .with_j_upper(29.5)
                            .with_cw(82500.0)
                            .with_wno(77.7)
                            .with_sw1(397.0)
                            .with_qf(82.3)
                            .with_qw(212.0)
                            .with_rts(5.03)
                            .with_ho(17.2)
                            .with_pa(86.6)
                            .with_pb(105.0)
                            .with_pc(54.7)
                            .with_pd(72.8)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?detailing_depth=18.25").await;

        response.assert_status_ok();
        let piles: Vec<HPile> = response.json::<Vec<HPile>>();
        assert_eq!(1, piles.iter().count());
        assert_eq!(
            String::from("HP18X204"),
            piles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(18.25, piles.iter().nth(0).unwrap().ddet);
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(18.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X181"))
                            .with_aisc_manual_label(String::from("HP18X181"))
                            .with_w_upper(181.0)
                            .with_a_upper(53.2)
                            .with_d_lower(18.0)
                            .with_ddet(18.0)
                            .with_bf(18.0)
                            .with_bfdet(18.0)
                            .with_tw(1.0)
                            .with_twdet(1.0)
                            .with_twdet_2(0.5)
                            .with_tf(1.0)
                            .with_tfdet(1.0)
                            .with_kdes(2.18)
                            .with_kdet(2.1875)
                            .with_k1(1.6875)
                            .with_bf_2tf(9.0)
                            .with_h_tw(13.6)
                            .with_ix(3020.0)
                            .with_zx(379.0)
                            .with_sx(336.0)
                            .with_rx(7.53)
                            .with_iy(974.0)
                            .with_zy(167.0)
                            .with_sy(108.0)
                            .with_ry(4.28)
                            .with_j_upper(20.7)
                            .with_cw(70400.0)
                            .with_wno(76.5)
                            .with_sw1(344.0)
                            .with_qf(72.3)
                            .with_qw(185.0)
                            .with_rts(4.96)
                            .with_ho(17.0)
                            .with_pa(86.0)
                            .with_pb(104.0)
                            .with_pc(54.0)
                            .with_pd(72.0)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?detailing_flange_width=18.0").await;

        response.assert_status_ok();
        let piles: Vec<HPile> = response.json::<Vec<HPile>>();
        assert_eq!(1, piles.iter().count());
        assert_eq!(
            String::from("HP18X181"),
            piles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(18.0, piles.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(18.25))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X204"))
                            .with_aisc_manual_label(String::from("HP18X204"))
                            .with_w_upper(204.0)
                            .with_a_upper(60.2)
                            .with_d_lower(18.3)
                            .with_ddet(18.25)
                            .with_bf(18.1)
                            .with_bfdet(18.125)
                            .with_tw(1.13)
                            .with_twdet(1.125)
                            .with_twdet_2(0.5625)
                            .with_tf(1.13)
                            .with_tfdet(1.125)
                            .with_kdes(2.31)
                            .with_kdet(2.3125)
                            .with_k1(1.75)
                            .with_bf_2tf(8.01)
                            .with_h_tw(12.1)
                            .with_ix(3480.0)
                            .with_zx(433.0)
                            .with_sx(380.0)
                            .with_rx(7.6)
                            .with_iy(1120.0)
                            .with_zy(191.0)
                            .with_sy(124.0)
                            .with_ry(4.31)
                            .with_j_upper(29.5)
                            .with_cw(82500.0)
                            .with_wno(77.7)
                            .with_sw1(397.0)
                            .with_qf(82.3)
                            .with_qw(212.0)
                            .with_rts(5.03)
                            .with_ho(17.2)
                            .with_pa(86.6)
                            .with_pb(105.0)
                            .with_pc(54.7)
                            .with_pd(72.8)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                        // Not a real HP shape — synthetic test data sharing ddet=18.25
                        // with a different bfdet so the in-memory flange width filter can be exercised.
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X204_SYNTHETIC"))
                            .with_aisc_manual_label(String::from("HP18X204_SYNTHETIC"))
                            .with_w_upper(200.0)
                            .with_a_upper(58.8)
                            .with_d_lower(18.3)
                            .with_ddet(18.25)
                            .with_bf(16.0)
                            .with_bfdet(16.0)
                            .with_tw(1.13)
                            .with_twdet(1.125)
                            .with_twdet_2(0.5625)
                            .with_tf(1.13)
                            .with_tfdet(1.125)
                            .with_kdes(2.31)
                            .with_kdet(2.3125)
                            .with_k1(1.75)
                            .with_bf_2tf(7.08)
                            .with_h_tw(12.1)
                            .with_ix(3400.0)
                            .with_zx(420.0)
                            .with_sx(372.0)
                            .with_rx(7.6)
                            .with_iy(900.0)
                            .with_zy(172.0)
                            .with_sy(112.0)
                            .with_ry(3.91)
                            .with_j_upper(28.0)
                            .with_cw(75000.0)
                            .with_wno(70.0)
                            .with_sw1(380.0)
                            .with_qf(78.0)
                            .with_qw(205.0)
                            .with_rts(4.55)
                            .with_ho(17.2)
                            .with_pa(82.6)
                            .with_pb(101.0)
                            .with_pc(50.7)
                            .with_pd(68.8)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/h-piles?detailing_depth=18.25&detailing_flange_width=18.125")
            .await;

        response.assert_status_ok();
        let piles: Vec<HPile> = response.json::<Vec<HPile>>();
        assert_eq!(1, piles.iter().count());
        assert_eq!(
            String::from("HP18X204"),
            piles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(18.25, piles.iter().nth(0).unwrap().ddet);
        assert_eq!(18.125, piles.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(99.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(18.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("HP18X181"))
                            .with_aisc_manual_label(String::from("HP18X181"))
                            .with_w_upper(181.0)
                            .with_a_upper(53.2)
                            .with_d_lower(18.0)
                            .with_ddet(18.0)
                            .with_bf(18.0)
                            .with_bfdet(18.0)
                            .with_tw(1.0)
                            .with_twdet(1.0)
                            .with_twdet_2(0.5)
                            .with_tf(1.0)
                            .with_tfdet(1.0)
                            .with_kdes(2.18)
                            .with_kdet(2.1875)
                            .with_k1(1.6875)
                            .with_bf_2tf(9.0)
                            .with_h_tw(13.6)
                            .with_ix(3020.0)
                            .with_zx(379.0)
                            .with_sx(336.0)
                            .with_rx(7.53)
                            .with_iy(974.0)
                            .with_zy(167.0)
                            .with_sy(108.0)
                            .with_ry(4.28)
                            .with_j_upper(20.7)
                            .with_cw(70400.0)
                            .with_wno(76.5)
                            .with_sw1(344.0)
                            .with_qf(72.3)
                            .with_qw(185.0)
                            .with_rts(4.96)
                            .with_ho(17.0)
                            .with_pa(86.0)
                            .with_pb(104.0)
                            .with_pc(54.0)
                            .with_pd(72.0)
                            .with_t(13.5)
                            .with_wgi(7.5)
                            .try_build::<HP>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/h-piles?detailing_depth=99.0&detailing_flange_width=18.0")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockHPileRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockHPileRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HP18X204")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?aisc_manual_label=HP18X204").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HP18X204")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?aisc_manual_label=HP18X204").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HP18X204")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?edi_std_nomenclature=HP18X204").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HP18X204")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?edi_std_nomenclature=HP18X204").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(18.25_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?detailing_depth=18.25").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockHPileRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(18.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/h-piles", get(get_h_piles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/h-piles?detailing_flange_width=18.0").await;
        response.assert_status_internal_server_error();
    }
}
