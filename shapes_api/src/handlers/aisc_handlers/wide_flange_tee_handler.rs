use crate::dto;
use crate::dto::aisc_shapes::WideFlangeTee;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::WideFlangeTee as WFT;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for Wide Flange Tee Handler
pub struct AppStateDyn {
    /// Repository for wide flange tee shapes
    pub repo: Arc<dyn ShapeRepository<WFT>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC wide flange tee
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Tee Depth
    pub detailing_depth: Option<f64>,
    /// Tee Flange Width
    pub detailing_flange_width: Option<f64>,
}

/// Gets all wide flange tee AISC shapes
#[debug_handler]
pub async fn get_wide_flange_tees(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<WideFlangeTee>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<WideFlangeTee>>, AISCError> {
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

            let result: Vec<WideFlangeTee> =
                shapes.iter().map(|s: &WFT| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<WideFlangeTee>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::WideFlangeTee = s.into();
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
                let shape: dto::aisc_shapes::WideFlangeTee = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<WideFlangeTee>>, AISCError> {
    let mut tees: Vec<WideFlangeTee> = Vec::new();
    if let Some(depth) = params.detailing_depth {
        tees = get_from_detailing_depth(&state, depth, &mut tees).await?;
        if tees.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(flange_width) = params.detailing_flange_width {
        tees = get_from_detailing_flange_width(&state, flange_width, &mut tees).await?;
        if tees.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    Ok(AppJson(tees))
}

async fn get_from_detailing_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    tees: &mut Vec<WideFlangeTee>,
) -> Result<Vec<WideFlangeTee>, AISCError> {
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
    tees: &mut Vec<WideFlangeTee>,
) -> Result<Vec<WideFlangeTee>, AISCError> {
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

#[cfg(test)]
#[doc(hidden)]
pub mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use mockall::{mock, predicate};
    use shapes::aisc_shapes::shape_builder::ShapeBuilder;
    use shapes::aisc_shapes::shape_repository::ShapeRepository;
    use shapes::aisc_shapes::{MissingPropertyError, WideFlangeTee as WFT};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub WideFlangeTeeRepo {}

        impl ShapeRepository<WFT> for WideFlangeTeeRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WFT>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<WFT>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<WFT>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<WFT>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<WFT>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn wt12x31() -> WFT {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WFT>()
            .unwrap()
    }

    fn wt12x27_5() -> WFT {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X27.5"))
            .aisc_manual_label(String::from("WT12X27.5"))
            .t_f(false)
            .w_upper(27.5)
            .a_upper(8.10)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(8.02)
            .bfdet(8.0)
            .tw(0.39)
            .twdet(0.375)
            .twdet_2(0.1875)
            .tf(0.52)
            .tfdet(0.5)
            .kdes(1.02)
            .kdet(1.4375)
            .y_lower(3.41)
            .yp(1.25)
            .bf_2tf(7.71)
            .d_t(30.5)
            .ix(116.0)
            .zx(25.0)
            .sx(13.6)
            .rx(3.78)
            .iy(19.3)
            .zy(7.67)
            .sy(4.81)
            .ry(1.54)
            .j_upper(0.57)
            .cw(5.09)
            .ro(5.35)
            .h_upper(0.677)
            .pa(30.2)
            .pb(37.2)
            .pc(32.8)
            .pd(39.9)
            .wgi(4.0)
            .try_build::<WFT>()
            .unwrap()
    }

    fn wt4x29() -> WFT {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT4X29"))
            .aisc_manual_label(String::from("WT4X29"))
            .t_f(false)
            .w_upper(29.0)
            .a_upper(8.55)
            .d_lower(4.01)
            .ddet(4.0)
            .bf(8.22)
            .bfdet(8.25)
            .tw(0.36)
            .twdet(0.375)
            .twdet_2(0.1875)
            .tf(0.53)
            .tfdet(0.5)
            .kdes(0.93)
            .kdet(1.25)
            .y_lower(1.09)
            .yp(0.468)
            .bf_2tf(7.75)
            .d_t(11.1)
            .ix(5.57)
            .zx(2.94)
            .sx(1.84)
            .rx(0.807)
            .iy(12.2)
            .zy(4.73)
            .sy(2.97)
            .ry(1.19)
            .j_upper(0.536)
            .cw(1.87)
            .ro(3.29)
            .h_upper(0.888)
            .pa(16.2)
            .pb(24.5)
            .pc(24.9)
            .pd(33.2)
            .wgi(4.0)
            .try_build::<WFT>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![wt12x31(), wt12x27_5(), wt4x29()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flange-tees").await;

        response.assert_status_ok();
        let tees: Vec<WideFlangeTee> = response.json::<Vec<WideFlangeTee>>();
        assert_eq!(3, tees.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("WT12X31")))
            .returning(|_| Box::pin(async { Ok(Some(wt12x31())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?aisc_manual_label=WT12X31")
            .await;

        response.assert_status_ok();
        let tees: Vec<WideFlangeTee> = response.json::<Vec<WideFlangeTee>>();

        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("WT12X31"),
            tees.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("WT12X31")))
            .returning(|_| Box::pin(async { Ok(Some(wt12x31())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?edi_std_nomenclature=WT12X31")
            .await;

        response.assert_status_ok();
        let tees: Vec<WideFlangeTee> = response.json::<Vec<WideFlangeTee>>();

        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("WT12X31"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(11.875))
            .returning(|_| Box::pin(async { Ok(vec![wt12x31(), wt12x27_5()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flange-tees?detailing_depth=11.875").await;

        response.assert_status_ok();
        let tees: Vec<WideFlangeTee> = response.json::<Vec<WideFlangeTee>>();

        assert_eq!(2, tees.iter().count());
        tees.iter().for_each(|t| assert_eq!(11.875, t.ddet));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(7.0))
            .returning(|_| Box::pin(async { Ok(vec![wt12x31()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?detailing_flange_width=7.0")
            .await;

        response.assert_status_ok();
        let tees: Vec<WideFlangeTee> = response.json::<Vec<WideFlangeTee>>();

        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("WT12X31"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(7.0, tees.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockWideFlangeTeeRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(11.875))
            .returning(|_| Box::pin(async { Ok(vec![wt12x31(), wt12x27_5()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(7.0))
            .returning(|_| Box::pin(async { Ok(vec![wt12x31()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?detailing_depth=11.875&detailing_flange_width=7.0")
            .await;

        response.assert_status_ok();
        let tees: Vec<WideFlangeTee> = response.json::<Vec<WideFlangeTee>>();

        assert_eq!(1, tees.iter().count());
        tees.iter().for_each(|t| {
            assert_eq!(11.875, t.ddet);
            assert_eq!(7.0, t.bfdet);
        });
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockWideFlangeTeeRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(2.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(7.0))
            .returning(|_| Box::pin(async { Ok(vec![wt12x31()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?detailing_depth=2.0&detailing_flange_width=7.0")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flange-tees").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("Ix"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flange-tees").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("WT2X3.0")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?aisc_manual_label=WT2X3.0")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("WT12X31")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?aisc_manual_label=WT12X31")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("WT2X3.0")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?edi_std_nomenclature=WT2X3.0")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("WT12X31")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?edi_std_nomenclature=WT12X31")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(7.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/wide-flange-tees?detailing_flange_width=7.0")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockWideFlangeTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(11.875_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/wide-flange-tees", get(get_wide_flange_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/wide-flange-tees?detailing_depth=11.875").await;
        response.assert_status_internal_server_error();
    }
}
