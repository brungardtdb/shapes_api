use crate::dto;
use crate::dto::aisc_shapes::WideFlange;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::WideFlange as WF;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for Wide Flange Handler
pub struct AppStateDyn {
    /// Repository for wide flange shapes
    pub repo: Arc<dyn ShapeRepository<WF>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC wide flange
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

/// Gets all wide flange AISC shapes
#[debug_handler]
pub async fn get_wide_flanges(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<WideFlange>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<WideFlange>>, AISCError> {
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

            let result: Vec<WideFlange> = shapes.iter().map(|s: &WF| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<WideFlange>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::WideFlange = s.into();
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
                let shape: dto::aisc_shapes::WideFlange = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<WideFlange>>, AISCError> {
    let mut beams: Vec<WideFlange> = Vec::new();
    if let Some(depth) = params.detailing_depth {
        beams = get_from_detailing_depth(&state, depth, &mut beams).await?;
        if beams.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(flange_width) = params.detailing_flange_width {
        beams = get_from_detailing_flange_width(&state, flange_width, &mut beams).await?;
        if beams.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    Ok(AppJson(beams))
}

async fn get_from_detailing_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    beams: &mut Vec<WideFlange>,
) -> Result<Vec<WideFlange>, AISCError> {
    if beams.iter().nth(0).is_some() {
        return Ok(beams
            .iter()
            .filter(|b| b.ddet == depth)
            .map(|b| b.clone())
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
    beams: &mut Vec<WideFlange>,
) -> Result<Vec<WideFlange>, AISCError> {
    if beams.iter().nth(0).is_some() {
        return Ok(beams
            .iter()
            .filter(|b| b.bfdet == flange_width)
            .map(|b| b.clone())
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
    use shapes::aisc_shapes::{MissingPropertyError, WideFlange as WF};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub WideFlangeRepo {}

        impl ShapeRepository<WF> for WideFlangeRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WF>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<WF>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<WF>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<WF>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<WF>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn w44x408() -> WF {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("W44X408"))
            .aisc_manual_label(String::from("W44X408"))
            .t_f(true)
            .w_upper(408.0)
            .a_upper(120.0)
            .d_lower(44.8)
            .ddet(44.75)
            .bf(16.1)
            .bfdet(16.125)
            .tw(1.22)
            .twdet(1.25)
            .twdet_2(0.625)
            .tf(2.17)
            .tfdet(2.1875)
            .kdes(2.96)
            .kdet(3.375)
            .k1(1.8125)
            .bf_2tf(3.71)
            .h_tw(31.9)
            .ix(38700.0)
            .zx(2000.0)
            .sx(1730.0)
            .rx(18.0)
            .iy(1520.0)
            .zy(297.0)
            .sy(189.0)
            .ry(3.56)
            .j_upper(134.0)
            .cw(691000.0)
            .wno(172.0)
            .sw1(1500.0)
            .qf(344.0)
            .qw(994.0)
            .rts(4.33)
            .ho(42.6)
            .pa(134.0)
            .pb(150.0)
            .pc(106.0)
            .pd(122.0)
            .t(38.0)
            .wgi(5.5)
            .wgo(3.0)
            .try_build::<WF>()
            .unwrap()
    }

    fn w44x368() -> WF {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("W44X368"))
            .aisc_manual_label(String::from("W44X368"))
            .t_f(false)
            .w_upper(368.0)
            .a_upper(108.0)
            .d_lower(44.4)
            .ddet(44.375)
            .bf(16.0)
            .bfdet(16.0)
            .tw(1.1)
            .twdet(1.125)
            .twdet_2(0.5625)
            .tf(1.97)
            .tfdet(2.0)
            .kdes(2.76)
            .kdet(3.1875)
            .k1(1.75)
            .bf_2tf(4.06)
            .h_tw(35.4)
            .ix(34700.0)
            .zx(1800.0)
            .sx(1560.0)
            .rx(17.9)
            .iy(1350.0)
            .zy(265.0)
            .sy(169.0)
            .ry(3.54)
            .j_upper(100.0)
            .cw(608000.0)
            .wno(170.0)
            .sw1(1340.0)
            .qf(311.0)
            .qw(894.0)
            .rts(4.28)
            .ho(42.4)
            .pa(133.0)
            .pb(149.0)
            .pc(105.0)
            .pd(121.0)
            .t(38.0)
            .wgi(5.5)
            .wgo(3.0)
            .try_build::<WF>()
            .unwrap()
    }

    fn w40x397() -> WF {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("W40X397"))
            .aisc_manual_label(String::from("W40X397"))
            .t_f(true)
            .w_upper(397.0)
            .a_upper(117.0)
            .d_lower(41.0)
            .ddet(41.0)
            .bf(16.1)
            .bfdet(16.125)
            .tw(1.22)
            .twdet(1.25)
            .twdet_2(0.625)
            .tf(2.2)
            .tfdet(2.1875)
            .kdes(3.38)
            .kdet(3.5)
            .k1(1.8125)
            .bf_2tf(3.66)
            .h_tw(28.0)
            .ix(32000.0)
            .zx(1800.0)
            .sx(1560.0)
            .rx(16.6)
            .iy(1540.0)
            .zy(300.0)
            .sy(191.0)
            .ry(3.64)
            .j_upper(142.0)
            .cw(579000.0)
            .wno(156.0)
            .sw1(1380.0)
            .qf(318.0)
            .qw(891.0)
            .rts(4.38)
            .ho(38.8)
            .pa(126.0)
            .pb(142.0)
            .pc(98.1)
            .pd(114.0)
            .t(34.0)
            .wgi(7.5)
            .wgo(3.0)
            .try_build::<WF>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Ok(vec![w44x408(), w44x368(), w40x397()]) })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges").await;

        response.assert_status_ok();
        let beams: Vec<WideFlange> = response.json::<Vec<WideFlange>>();
        assert_eq!(3, beams.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("W44X408")))
            .returning(|_| Box::pin(async { Ok(Some(w44x408())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges?aisc_manual_label=W44X408").await;

        response.assert_status_ok();
        let beams: Vec<WideFlange> = response.json::<Vec<WideFlange>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("W44X408"),
            beams.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("W44X368")))
            .returning(|_| Box::pin(async { Ok(Some(w44x368())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges?edi_std_nomenclature=W44X368").await;

        response.assert_status_ok();
        let beams: Vec<WideFlange> = response.json::<Vec<WideFlange>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("W44X368"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(44.75_f64))
            .returning(|_| Box::pin(async { Ok(vec![w44x408()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges?detailing_depth=44.75").await;

        response.assert_status_ok();
        let beams: Vec<WideFlange> = response.json::<Vec<WideFlange>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("W44X408"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(44.75, beams.iter().nth(0).unwrap().ddet);
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(16.125_f64))
            .returning(|_| Box::pin(async { Ok(vec![w44x408(), w40x397()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flanges?detailing_flange_width=16.125")
            .await;

        response.assert_status_ok();
        let beams: Vec<WideFlange> = response.json::<Vec<WideFlange>>();

        assert_eq!(2, beams.iter().count());
        beams.iter().for_each(|b| assert_eq!(16.125, b.bfdet));
    }

    #[tokio::test]
    async fn filters_on_depth_and_width() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(44.75_f64))
            .returning(|_| Box::pin(async { Ok(vec![w44x408(), w44x368()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(16.125_f64))
            .returning(|_| Box::pin(async { Ok(vec![w44x408(), w40x397()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flanges?detailing_depth=44.75&detailing_flange_width=16.125")
            .await;

        response.assert_status_ok();
        let beams: Vec<WideFlange> = response.json::<Vec<WideFlange>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("W44X408"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(44.75, beams.iter().nth(0).unwrap().ddet);
        assert_eq!(16.125, beams.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_depth_and_width_with_one_query_returning_no_records() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(99.0_f64))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flanges?detailing_depth=99.0&detailing_flange_width=16.125")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Err(MissingPropertyError::from("ddet"))? })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("W44X999")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges?aisc_manual_label=W44X999").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("W44X408")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges?aisc_manual_label=W44X408").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("W44X999")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flanges?edi_std_nomenclature=W44X999")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("W44X368")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("W"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flanges?edi_std_nomenclature=W44X368")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(16.125_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flanges?detailing_flange_width=16.125")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockWideFlangeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(44.75_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flanges", get(get_wide_flanges))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flanges?detailing_depth=44.75").await;
        response.assert_status_internal_server_error();
    }
}
