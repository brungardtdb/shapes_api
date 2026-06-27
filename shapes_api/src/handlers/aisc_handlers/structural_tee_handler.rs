use crate::dto;
use crate::dto::aisc_shapes::StructuralTee;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::StructuralTee as ST;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for Structural Tee Handler
pub struct AppStateDyn {
    /// Repository for structural tee shapes
    pub repo: Arc<dyn ShapeRepository<ST>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC structural tee
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

/// Gets all structural tee AISC shapes
#[debug_handler]
pub async fn get_structural_tees(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<StructuralTee>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<StructuralTee>>, AISCError> {
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

            let result: Vec<StructuralTee> =
                shapes.iter().map(|s: &ST| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<StructuralTee>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::StructuralTee = s.into();
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
                let shape: dto::aisc_shapes::StructuralTee = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<StructuralTee>>, AISCError> {
    let mut tees: Vec<StructuralTee> = Vec::new();
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
    tees: &mut Vec<StructuralTee>,
) -> Result<Vec<StructuralTee>, AISCError> {
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
    tees: &mut Vec<StructuralTee>,
) -> Result<Vec<StructuralTee>, AISCError> {
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
    use shapes::aisc_shapes::{MissingPropertyError, StructuralTee as ST};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub StructuralTeeRepo {}

        impl ShapeRepository<ST> for StructuralTeeRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<ST>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<ST>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<ST>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<ST>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<ST>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn st6x20_4() -> ST {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("ST6X20.4"))
            .aisc_manual_label(String::from("ST6X20.4"))
            .w_upper(20.4)
            .a_upper(5.96)
            .d_lower(6.00)
            .ddet(6.00)
            .bf(5.25)
            .bfdet(5.25)
            .tw(0.462)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.659)
            .tfdet(0.6875)
            .kdes(1.44)
            .kdet(1.4375)
            .y_lower(1.58)
            .yp(0.577)
            .bf_2tf(3.98)
            .d_t(13.0)
            .ix(18.9)
            .zx(7.71)
            .sx(4.27)
            .rx(1.78)
            .iy(6.74)
            .zy(4.43)
            .sy(2.57)
            .ry(1.00)
            .j_upper(0.842)
            .cw(0.787)
            .ro(2.42)
            .h_upper(0.732)
            .wgi(3.0)
            .try_build::<ST>()
            .unwrap()
    }

    fn st6x17_5() -> ST {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("ST6X17.5"))
            .aisc_manual_label(String::from("ST6X17.5"))
            .w_upper(17.5)
            .a_upper(5.07)
            .d_lower(6.00)
            .ddet(6.00)
            .bf(5.0)
            .bfdet(5.0)
            .tw(0.428)
            .twdet(0.4375)
            .twdet_2(0.21875)
            .tf(0.544)
            .tfdet(0.5625)
            .kdes(1.19)
            .kdet(1.1875)
            .y_lower(1.50)
            .yp(0.520)
            .bf_2tf(4.60)
            .d_t(14.0)
            .ix(16.0)
            .zx(6.50)
            .sx(3.60)
            .rx(1.77)
            .iy(5.52)
            .zy(3.80)
            .sy(2.21)
            .ry(1.04)
            .j_upper(0.530)
            .cw(0.620)
            .ro(2.38)
            .h_upper(0.751)
            .try_build::<ST>()
            .unwrap()
    }

    fn st4x9_2() -> ST {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("ST4X9.2"))
            .aisc_manual_label(String::from("ST4X9.2"))
            .w_upper(9.2)
            .a_upper(2.69)
            .d_lower(4.00)
            .ddet(4.00)
            .bf(3.5)
            .bfdet(3.5)
            .tw(0.271)
            .twdet(0.3125)
            .twdet_2(0.15625)
            .tf(0.425)
            .tfdet(0.4375)
            .kdes(0.891)
            .kdet(0.875)
            .y_lower(0.977)
            .yp(0.311)
            .bf_2tf(4.12)
            .d_t(14.8)
            .ix(4.08)
            .zx(1.94)
            .sx(1.40)
            .rx(1.23)
            .iy(1.55)
            .zy(1.52)
            .sy(0.886)
            .ry(0.759)
            .j_upper(0.122)
            .cw(0.0558)
            .ro(1.55)
            .h_upper(0.757)
            .try_build::<ST>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![st6x20_4(), st6x17_5(), st4x9_2()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-tees").await;

        response.assert_status_ok();
        let tees: Vec<StructuralTee> = response.json::<Vec<StructuralTee>>();
        assert_eq!(3, tees.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("ST6X20.4")))
            .returning(|_| Box::pin(async { Ok(Some(st6x20_4())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?aisc_manual_label=ST6X20.4")
            .await;

        response.assert_status_ok();
        let tees: Vec<StructuralTee> = response.json::<Vec<StructuralTee>>();

        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("ST6X20.4"),
            tees.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("ST6X20.4")))
            .returning(|_| Box::pin(async { Ok(Some(st6x20_4())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?edi_std_nomenclature=ST6X20.4")
            .await;

        response.assert_status_ok();
        let tees: Vec<StructuralTee> = response.json::<Vec<StructuralTee>>();

        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("ST6X20.4"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.0))
            .returning(|_| Box::pin(async { Ok(vec![st6x20_4(), st6x17_5()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-tees?detailing_depth=6.0").await;

        response.assert_status_ok();
        let tees: Vec<StructuralTee> = response.json::<Vec<StructuralTee>>();

        assert_eq!(2, tees.iter().count());
        tees.iter().for_each(|t| assert_eq!(6.0, t.ddet));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(5.25))
            .returning(|_| Box::pin(async { Ok(vec![st6x20_4()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?detailing_flange_width=5.25")
            .await;

        response.assert_status_ok();
        let tees: Vec<StructuralTee> = response.json::<Vec<StructuralTee>>();

        assert_eq!(1, tees.iter().count());
        assert_eq!(
            String::from("ST6X20.4"),
            tees.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(5.25, tees.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockStructuralTeeRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.0))
            .returning(|_| Box::pin(async { Ok(vec![st6x20_4(), st6x17_5()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(5.25))
            .returning(|_| Box::pin(async { Ok(vec![st6x20_4()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?detailing_depth=6.0&detailing_flange_width=5.25")
            .await;

        response.assert_status_ok();
        let tees: Vec<StructuralTee> = response.json::<Vec<StructuralTee>>();

        assert_eq!(1, tees.iter().count());
        tees.iter().for_each(|t| {
            assert_eq!(6.0, t.ddet);
            assert_eq!(5.25, t.bfdet);
        });
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockStructuralTeeRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(2.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(5.25))
            .returning(|_| Box::pin(async { Ok(vec![st6x20_4()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?detailing_depth=2.0&detailing_flange_width=5.25")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-tees").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("Ix"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-tees").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("ST2X3.0")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?aisc_manual_label=ST2X3.0")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("ST6X20.4")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?aisc_manual_label=ST6X20.4")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("ST2X3.0")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?edi_std_nomenclature=ST2X3.0")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("ST6X20.4")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?edi_std_nomenclature=ST6X20.4")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(5.25_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-tees?detailing_flange_width=5.25")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockStructuralTeeRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-tees", get(get_structural_tees))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-tees?detailing_depth=6.0").await;
        response.assert_status_internal_server_error();
    }
}
