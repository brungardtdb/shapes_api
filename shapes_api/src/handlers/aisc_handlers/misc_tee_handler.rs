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
pub async fn get_misc_tees(
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

#[cfg(test)]
#[doc(hidden)]
pub mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use mockall::{mock, predicate};
    use shapes::aisc_shapes::shape_builder::ShapeBuilder;
    use shapes::aisc_shapes::shape_repository::ShapeRepository;
    use shapes::aisc_shapes::{MiscTee as MT, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub MiscTeeRepo {}

        impl ShapeRepository<MT> for MiscTeeRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<MT>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<MT>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<MT>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<MT>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<MT>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn mt6_25x6_2() -> MT {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("MT6.25X6.2"))
            .aisc_manual_label(String::from("MT6.25X6.2"))
            .t_f(false)
            .w_upper(6.2)
            .a_upper(1.82)
            .d_lower(6.27)
            .ddet(6.25)
            .bf(3.75)
            .bfdet(3.75)
            .tw(0.155)
            .twdet(0.125)
            .twdet_2(0.0625)
            .tf(0.228)
            .tfdet(0.25)
            .kdes(0.563)
            .kdet(0.5625)
            .y_lower(1.74)
            .yp(0.372)
            .bf_2tf(8.22)
            .d_t(40.4)
            .ix(7.29)
            .zx(2.92)
            .sx(1.61)
            .rx(2.01)
            .iy(1.0)
            .zy(0.839)
            .sy(0.536)
            .ry(0.746)
            .j_upper(0.0246)
            .cw(0.0284)
            .ro(2.69)
            .h_upper(0.634)
            .try_build::<MT>()
            .unwrap()
    }

    fn mt6_25x5_8() -> MT {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("MT6.25X5.8"))
            .aisc_manual_label(String::from("MT6.25X5.8"))
            .t_f(false)
            .w_upper(5.8)
            .a_upper(1.7)
            .d_lower(6.25)
            .ddet(6.25)
            .bf(3.5)
            .bfdet(3.5)
            .tw(0.155)
            .twdet(0.125)
            .twdet_2(0.0625)
            .tf(0.211)
            .tfdet(0.1875)
            .kdes(0.563)
            .kdet(0.5625)
            .y_lower(1.84)
            .yp(0.808)
            .bf_2tf(8.29)
            .d_t(40.3)
            .ix(6.94)
            .zx(2.86)
            .sx(1.57)
            .rx(2.03)
            .iy(0.756)
            .zy(0.684)
            .sy(0.432)
            .ry(0.669)
            .j_upper(0.0206)
            .cw(0.0268)
            .ro(2.75)
            .h_upper(0.602)
            .try_build::<MT>()
            .unwrap()
    }

    fn mt6x5_9() -> MT {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("MT6X5.9"))
            .aisc_manual_label(String::from("MT6X5.9"))
            .t_f(false)
            .w_upper(5.9)
            .a_upper(1.74)
            .d_lower(6.0)
            .ddet(6.0)
            .bf(3.07)
            .bfdet(3.125)
            .tw(0.177)
            .twdet(0.1875)
            .twdet_2(0.125)
            .tf(0.225)
            .tfdet(0.25)
            .kdes(0.563)
            .kdet(0.5625)
            .y_lower(1.89)
            .yp(1.13)
            .bf_2tf(6.82)
            .d_t(33.9)
            .ix(6.61)
            .zx(2.89)
            .sx(1.61)
            .rx(1.96)
            .iy(0.543)
            .zy(0.575)
            .sy(0.354)
            .ry(0.561)
            .j_upper(0.0249)
            .cw(0.0337)
            .ro(2.71)
            .h_upper(0.567)
            .try_build::<MT>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![mt6_25x6_2(), mt6_25x5_8(), mt6x5_9()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees").await;

        response.assert_status_ok();
        let tees: Vec<MiscTee> = response.json::<Vec<MiscTee>>();
        assert_eq!(3, tees.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("MT6.25X6.2")))
            .returning(|_| Box::pin(async { Ok(Some(mt6_25x6_2())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?aisc_manual_label=MT6.25X6.2").await;

        response.assert_status_ok();
        let tees: Vec<MiscTee> = response.json::<Vec<MiscTee>>();
        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("MT6.25X6.2"),
            tees.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("MT6.25X6.2")))
            .returning(|_| Box::pin(async { Ok(Some(mt6_25x6_2())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-tees?edi_std_nomenclature=MT6.25X6.2")
            .await;

        response.assert_status_ok();
        let tees: Vec<MiscTee> = response.json::<Vec<MiscTee>>();
        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("MT6.25X6.2"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.25))
            .returning(|_| Box::pin(async { Ok(vec![mt6_25x6_2(), mt6_25x5_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?detailing_depth=6.25").await;

        response.assert_status_ok();
        let tees: Vec<MiscTee> = response.json::<Vec<MiscTee>>();
        assert_eq!(2, tees.iter().count());
        tees.iter().for_each(|t| assert_eq!(6.25, t.ddet));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(3.75))
            .returning(|_| Box::pin(async { Ok(vec![mt6_25x6_2()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?detailing_flange_width=3.75").await;

        response.assert_status_ok();
        let tees: Vec<MiscTee> = response.json::<Vec<MiscTee>>();
        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("MT6.25X6.2"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(3.75, tees.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.25))
            .returning(|_| Box::pin(async { Ok(vec![mt6_25x6_2(), mt6_25x5_8()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(3.5))
            .returning(|_| Box::pin(async { Ok(vec![mt6_25x5_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-tees?detailing_depth=6.25&detailing_flange_width=3.5")
            .await;

        response.assert_status_ok();
        let tees: Vec<MiscTee> = response.json::<Vec<MiscTee>>();
        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("MT6.25X5.8"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(6.25, tees.iter().nth(0).unwrap().ddet);
        assert_eq!(3.5, tees.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(1.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(3.5))
            .returning(|_| Box::pin(async { Ok(vec![mt6_25x5_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-tees?detailing_depth=1.0&detailing_flange_width=3.5")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("MT6.25X6.2")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?aisc_manual_label=MT6.25X6.2").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("MT6.25X6.2")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?aisc_manual_label=MT6.25X6.2").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("MT6.25X6.2")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-tees?edi_std_nomenclature=MT6.25X6.2")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("MT6.25X6.2")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-tees?edi_std_nomenclature=MT6.25X6.2")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(3.75_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?detailing_flange_width=3.75").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockMiscTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.25_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-tees", get(get_misc_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-tees?detailing_depth=6.25").await;
        response.assert_status_internal_server_error();
    }
}
