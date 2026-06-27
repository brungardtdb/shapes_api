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
        if sections.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(width) = params.member_width {
        sections = get_from_member_width(&state, width, &mut sections).await?;
        if sections.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
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

#[cfg(test)]
#[doc(hidden)]
pub mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use axum_test::TestServer;
    use mockall::{mock, predicate};
    use shapes::aisc_shapes::shape_builder::ShapeBuilder;
    use shapes::aisc_shapes::shape_repository::ShapeRepository;
    use shapes::aisc_shapes::{HollowStructuralSection as HSS, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub HSSRepo {}

        impl ShapeRepository<HSS> for HSSRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<HSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<HSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<HSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<HSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<HSS>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn hss_8x6x500() -> HSS {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("HSS8X6X.500"))
            .aisc_manual_label(String::from("HSS8X6X1/2"))
            .w_upper(42.05)
            .a_upper(11.6)
            .ht(8.0)
            .h(6.6)
            .b_upper(6.0)
            .b_lower(4.61)
            .t_nom(0.5)
            .tdes(0.465)
            .b_tdes(9.9)
            .h_tdes(14.2)
            .ix(98.2)
            .zx(30.5)
            .sx(24.6)
            .rx(2.91)
            .iy(62.5)
            .zy(24.9)
            .sy(20.8)
            .ry(2.32)
            .j_upper(127.0)
            .c_upper(38.4)
            .try_build::<HSS>()
            .unwrap()
    }

    fn hss_8x8x500() -> HSS {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("HSS8X8X.500"))
            .aisc_manual_label(String::from("HSS8X8X1/2"))
            .w_upper(48.85)
            .a_upper(13.5)
            .ht(8.0)
            .h(6.6)
            .b_upper(8.0)
            .b_lower(6.6)
            .t_nom(0.5)
            .tdes(0.465)
            .b_tdes(14.2)
            .h_tdes(14.2)
            .ix(125.0)
            .zx(37.5)
            .sx(31.2)
            .rx(3.04)
            .iy(125.0)
            .zy(37.5)
            .sy(31.2)
            .ry(3.04)
            .j_upper(204.0)
            .c_upper(52.4)
            .try_build::<HSS>()
            .unwrap()
    }

    fn hss_6x6x500() -> HSS {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("HSS6X6X.500"))
            .aisc_manual_label(String::from("HSS6X6X1/2"))
            .w_upper(35.24)
            .a_upper(9.74)
            .ht(6.0)
            .h(4.61)
            .b_upper(6.0)
            .b_lower(4.61)
            .t_nom(0.5)
            .tdes(0.465)
            .b_tdes(9.9)
            .h_tdes(9.9)
            .ix(48.3)
            .zx(19.8)
            .sx(16.1)
            .rx(2.23)
            .iy(48.3)
            .zy(19.8)
            .sy(16.1)
            .ry(2.23)
            .j_upper(81.1)
            .c_upper(28.1)
            .try_build::<HSS>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockHSSRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Ok(vec![hss_8x6x500(), hss_8x8x500(), hss_6x6x500()]) })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss").await;

        response.assert_status_ok();
        let sections: Vec<HollowStructuralSection> =
            response.json::<Vec<HollowStructuralSection>>();
        assert_eq!(3, sections.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HSS8X6X1/2")))
            .returning(|_| Box::pin(async { Ok(Some(hss_8x6x500())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?aisc_manual_label=HSS8X6X1/2").await;

        response.assert_status_ok();
        let sections: Vec<HollowStructuralSection> =
            response.json::<Vec<HollowStructuralSection>>();

        assert_eq!(1, sections.iter().count());
        assert_eq!(
            String::from("HSS8X6X1/2"),
            sections.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HSS8X6X.500")))
            .returning(|_| Box::pin(async { Ok(Some(hss_8x6x500())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?edi_std_nomenclature=HSS8X6X.500").await;

        response.assert_status_ok();
        let sections: Vec<HollowStructuralSection> =
            response.json::<Vec<HollowStructuralSection>>();

        assert_eq!(1, sections.iter().count());
        assert_eq!(
            String::from("HSS8X6X.500"),
            sections.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0))
            .returning(|_| Box::pin(async { Ok(vec![hss_8x6x500(), hss_8x8x500()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?member_depth=8.0").await;

        response.assert_status_ok();
        let sections: Vec<HollowStructuralSection> =
            response.json::<Vec<HollowStructuralSection>>();

        assert_eq!(2, sections.iter().count());
        sections.iter().for_each(|s| assert_eq!(8.0, s.ht));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(6.0))
            .returning(|_| Box::pin(async { Ok(vec![hss_8x6x500(), hss_6x6x500()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?member_width=6.0").await;

        response.assert_status_ok();
        let sections: Vec<HollowStructuralSection> =
            response.json::<Vec<HollowStructuralSection>>();

        assert_eq!(2, sections.iter().count());
        sections.iter().for_each(|s| assert_eq!(6.0, s.b_upper));
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0))
            .returning(|_| Box::pin(async { Ok(vec![hss_8x6x500(), hss_8x8x500()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?member_depth=8.0&member_width=6.0").await;

        response.assert_status_ok();
        let sections: Vec<HollowStructuralSection> =
            response.json::<Vec<HollowStructuralSection>>();

        assert_eq!(1, sections.iter().count());
        assert_eq!(
            String::from("HSS8X6X.500"),
            sections.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(8.0, sections.iter().nth(0).unwrap().ht);
        assert_eq!(6.0, sections.iter().nth(0).unwrap().b_upper);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(8.0))
            .returning(|_| Box::pin(async { Ok(vec![hss_8x8x500()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?member_depth=6.0&member_width=8.0").await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockHSSRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockHSSRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("Ht"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HSS8X6X1/2")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?aisc_manual_label=HSS8X6X1/2").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HSS8X6X1/2")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("Ht"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?aisc_manual_label=HSS8X6X1/2").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HSS8X6X.500")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?edi_std_nomenclature=HSS8X6X.500").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HSS8X6X.500")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("Ht"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?edi_std_nomenclature=HSS8X6X.500").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("Ht"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?member_depth=8.0").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockHSSRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(6.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("B"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/hss", get(get_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/hss?member_width=6.0").await;
        response.assert_status_internal_server_error();
    }
}
