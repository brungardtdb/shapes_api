use crate::dto;
use crate::dto::aisc_shapes::RoundHollowStructuralSection;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::RoundHollowStructuralSection as RoundHSS;
use shapes::aisc_shapes::shape_repository::RoundShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Round Hollow Structural Section Handler
pub struct AppStateDyn {
    /// Repository for round HSS shapes
    pub repo: Arc<dyn RoundShapeRepository<RoundHSS>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC round hollow structural sections
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Outside Diameter (OD)
    pub diameter: Option<f64>,
}

/// Gets all round hollow structural section AISC shapes
#[debug_handler]
pub async fn get_round_hollow_structural_sections(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<RoundHollowStructuralSection>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(
    state: Arc<AppStateDyn>,
) -> Result<AppJson<Vec<RoundHollowStructuralSection>>, AISCError> {
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

            let result: Vec<RoundHollowStructuralSection> = shapes
                .iter()
                .map(|s: &RoundHSS| s.into())
                .collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<RoundHollowStructuralSection>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::RoundHollowStructuralSection = s.into();
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
                let shape: dto::aisc_shapes::RoundHollowStructuralSection = s.into();
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
    use shapes::aisc_shapes::{MissingPropertyError, RoundHollowStructuralSection as RoundHSS};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub RoundHSSRepo {}

        impl RoundShapeRepository<RoundHSS> for RoundHSSRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<RoundHSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<RoundHSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<RoundHSS>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_diameter<'a>(
                &'a self,
                diameter: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<RoundHSS>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn hss_10x625() -> RoundHSS {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("HSS10X.625"))
            .aisc_manual_label(String::from("HSS10.000X0.625"))
            .w_upper(62.64)
            .a_upper(17.2)
            .od(10.0)
            .t_nom(0.625)
            .tdes(0.581)
            .d_t(17.2)
            .ix(191.0)
            .zx(51.6)
            .sx(38.3)
            .rx(3.34)
            .iy(191.0)
            .zy(51.6)
            .sy(38.3)
            .ry(3.34)
            .j_upper(383.0)
            .c_upper(76.6)
            .try_build::<RoundHSS>()
            .unwrap()
    }

    fn hss_10x500() -> RoundHSS {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("HSS10X.500"))
            .aisc_manual_label(String::from("HSS10.000X0.500"))
            .w_upper(51.16)
            .a_upper(14.1)
            .od(10.0)
            .t_nom(0.5)
            .tdes(0.465)
            .d_t(21.5)
            .ix(160.0)
            .zx(42.7)
            .sx(32.0)
            .rx(3.37)
            .iy(160.0)
            .zy(42.7)
            .sy(32.0)
            .ry(3.37)
            .j_upper(320.0)
            .c_upper(64.0)
            .try_build::<RoundHSS>()
            .unwrap()
    }

    fn hss_8x500() -> RoundHSS {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("HSS8X.500"))
            .aisc_manual_label(String::from("HSS8.625X0.500"))
            .w_upper(43.39)
            .a_upper(11.9)
            .od(8.625)
            .t_nom(0.5)
            .tdes(0.465)
            .d_t(18.5)
            .ix(97.2)
            .zx(29.7)
            .sx(22.6)
            .rx(2.86)
            .iy(97.2)
            .zy(29.7)
            .sy(22.6)
            .ry(2.86)
            .j_upper(194.0)
            .c_upper(45.1)
            .try_build::<RoundHSS>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![hss_10x625(), hss_10x500(), hss_8x500()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/round-hss").await;

        response.assert_status_ok();
        let sections: Vec<RoundHollowStructuralSection> =
            response.json::<Vec<RoundHollowStructuralSection>>();
        assert_eq!(3, sections.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HSS10.000X0.625")))
            .returning(|_| Box::pin(async { Ok(Some(hss_10x625())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/round-hss?aisc_manual_label=HSS10.000X0.625")
            .await;

        response.assert_status_ok();
        let sections: Vec<RoundHollowStructuralSection> =
            response.json::<Vec<RoundHollowStructuralSection>>();

        assert_eq!(1, sections.iter().count());
        assert_eq!(
            String::from("HSS10.000X0.625"),
            sections.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HSS10X.625")))
            .returning(|_| Box::pin(async { Ok(Some(hss_10x625())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/round-hss?edi_std_nomenclature=HSS10X.625")
            .await;

        response.assert_status_ok();
        let sections: Vec<RoundHollowStructuralSection> =
            response.json::<Vec<RoundHollowStructuralSection>>();

        assert_eq!(1, sections.iter().count());
        assert_eq!(
            String::from("HSS10X.625"),
            sections.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_diameter() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shapes_with_diameter()
            .with(predicate::eq(10.0))
            .returning(|_| Box::pin(async { Ok(vec![hss_10x625(), hss_10x500()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/round-hss?diameter=10.0").await;

        response.assert_status_ok();
        let sections: Vec<RoundHollowStructuralSection> =
            response.json::<Vec<RoundHollowStructuralSection>>();

        assert_eq!(2, sections.iter().count());
        sections.iter().for_each(|s| assert_eq!(10.0, s.od));
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/round-hss").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/round-hss").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HSS10.000X0.625")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/round-hss?aisc_manual_label=HSS10.000X0.625")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("HSS10.000X0.625")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/round-hss?aisc_manual_label=HSS10.000X0.625")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HSS10X.625")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/round-hss?edi_std_nomenclature=HSS10X.625")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("HSS10X.625")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/round-hss?edi_std_nomenclature=HSS10X.625")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_diameter() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shapes_with_diameter()
            .with(predicate::eq(10.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("OD"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/round-hss?diameter=10.0").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn returns_not_found_when_no_shapes_match_diameter() {
        let mut repo = MockRoundHSSRepo::new();
        repo.expect_shapes_with_diameter()
            .with(predicate::eq(99.0_f64))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/round-hss", get(get_round_hollow_structural_sections))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/round-hss?diameter=99.0").await;
        response.assert_status_not_found();
    }
}
