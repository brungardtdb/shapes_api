use crate::dto;
use crate::dto::aisc_shapes::Pipe;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::Pipe as AISCPipe;
use shapes::aisc_shapes::shape_repository::RoundShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Pipe Handler
pub struct AppStateDyn {
    /// Repository for pipe shapes
    pub repo: Arc<dyn RoundShapeRepository<AISCPipe>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC pipes
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Outside Diameter (OD)
    pub diameter: Option<f64>,
}

/// Gets all pipe AISC shapes
#[debug_handler]
pub async fn get_pipes(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<Pipe>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<Pipe>>, AISCError> {
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

            let result: Vec<Pipe> = shapes
                .iter()
                .map(|s: &AISCPipe| s.into())
                .collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<Pipe>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::Pipe = s.into();
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
                let shape: dto::aisc_shapes::Pipe = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    // Check for diameter
    if let Some(diameter) = params.diameter {
        let result = state.repo.shapes_with_diameter(diameter).await;
        match result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(shapes) => {
                if shapes.is_empty() {
                    return Err(AISCError::ShapeNotFound);
                }
                return Ok(AppJson(shapes.iter().map(|s| s.into()).collect::<Vec<_>>()));
            }
        }
    }
    Ok(AppJson(vec![]))
}

fn has_query(params: &Params) -> bool {
    if let Some(_) = params.aisc_manual_label {
        return true;
    }
    if let Some(_) = params.edi_std_nomenclature {
        return true;
    }
    if let Some(_) = params.diameter {
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
    use shapes::aisc_shapes::shape_repository::RoundShapeRepository;
    use shapes::aisc_shapes::{MissingPropertyError, Pipe as AISCPipe};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub PipeRepo {}

        impl RoundShapeRepository<AISCPipe> for PipeRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<AISCPipe>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<AISCPipe>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<AISCPipe>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_diameter<'a>(
                &'a self,
                diameter: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<AISCPipe>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn pipe_26_std() -> AISCPipe {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("Pipe26STD"))
            .aisc_manual_label(String::from("Pipe26STD"))
            .w_upper(103.0)
            .a_upper(28.2)
            .od(26.0)
            .id(25.3)
            .t_nom(0.375)
            .tdes(0.349)
            .d_t(74.5)
            .ix(2320.0)
            .zx(230.0)
            .sx(178.0)
            .rx(9.07)
            .iy(191.0)
            .zy(230.0)
            .sy(178.0)
            .ry(9.07)
            .j_upper(4640.0)
            .try_build::<AISCPipe>()
            .unwrap()
    }

    fn pipe_24_std() -> AISCPipe {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("Pipe24STD"))
            .aisc_manual_label(String::from("Pipe24STD"))
            .w_upper(94.62)
            .a_upper(26.0)
            .od(24.0)
            .id(23.25)
            .t_nom(0.375)
            .tdes(0.349)
            .d_t(68.8)
            .ix(1830.0)
            .zx(196.0)
            .sx(152.0)
            .rx(8.39)
            .iy(153.0)
            .zy(196.0)
            .sy(152.0)
            .ry(8.39)
            .j_upper(3660.0)
            .try_build::<AISCPipe>()
            .unwrap()
    }

    fn pipe_20_std() -> AISCPipe {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("Pipe20STD"))
            .aisc_manual_label(String::from("Pipe20STD"))
            .w_upper(78.6)
            .a_upper(21.6)
            .od(20.0)
            .id(19.25)
            .t_nom(0.375)
            .tdes(0.349)
            .d_t(57.3)
            .ix(1050.0)
            .zx(135.0)
            .sx(105.0)
            .rx(6.97)
            .iy(87.6)
            .zy(135.0)
            .sy(105.0)
            .ry(6.97)
            .j_upper(2100.0)
            .try_build::<AISCPipe>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockPipeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![pipe_26_std(), pipe_24_std(), pipe_20_std()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe").await;

        response.assert_status_ok();
        let pipes: Vec<Pipe> = response.json::<Vec<Pipe>>();
        assert_eq!(3, pipes.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("Pipe26STD")))
            .returning(|_| Box::pin(async { Ok(Some(pipe_26_std())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?aisc_manual_label=Pipe26STD").await;

        response.assert_status_ok();
        let pipes: Vec<Pipe> = response.json::<Vec<Pipe>>();

        assert_eq!(1, pipes.iter().count());
        assert_eq!(
            String::from("Pipe26STD"),
            pipes.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("Pipe26STD")))
            .returning(|_| Box::pin(async { Ok(Some(pipe_26_std())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?edi_std_nomenclature=Pipe26STD").await;

        response.assert_status_ok();
        let pipes: Vec<Pipe> = response.json::<Vec<Pipe>>();

        assert_eq!(1, pipes.iter().count());
        assert_eq!(
            String::from("Pipe26STD"),
            pipes.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_diameter() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shapes_with_diameter()
            .with(predicate::eq(26.0))
            .returning(|_| Box::pin(async { Ok(vec![pipe_26_std()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?diameter=26.0").await;

        response.assert_status_ok();
        let pipes: Vec<Pipe> = response.json::<Vec<Pipe>>();

        assert_eq!(1, pipes.iter().count());
        pipes.iter().for_each(|s| assert_eq!(26.0, s.od));
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockPipeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockPipeRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("Pipe26STD")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?aisc_manual_label=Pipe26STD").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("Pipe26STD")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?aisc_manual_label=Pipe26STD").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("Pipe26STD")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?edi_std_nomenclature=Pipe26STD").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("Pipe26STD")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?edi_std_nomenclature=Pipe26STD").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_diameter() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shapes_with_diameter()
            .with(predicate::eq(26.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?diameter=26.0").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn returns_not_found_when_no_shapes_match_diameter() {
        let mut repo = MockPipeRepo::new();
        repo.expect_shapes_with_diameter()
            .with(predicate::eq(99.0_f64))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/pipe", get(get_pipes))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/pipe?diameter=99.0").await;
        response.assert_status_not_found();
    }
}
