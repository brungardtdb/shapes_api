use crate::dto;
use crate::dto::aisc_shapes::StructuralBeam;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::StructuralBeam as SB;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for Structural Beam Handler
pub struct AppStateDyn {
    /// Repository for structural beam shapes
    pub repo: Arc<dyn ShapeRepository<SB>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC structural beam
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

/// Gets all structural beam AISC shapes
#[debug_handler]
pub async fn get_structural_beams(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<StructuralBeam>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<StructuralBeam>>, AISCError> {
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

            let result: Vec<StructuralBeam> =
                shapes.iter().map(|s: &SB| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<StructuralBeam>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::StructuralBeam = s.into();
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
                let shape: dto::aisc_shapes::StructuralBeam = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<StructuralBeam>>, AISCError> {
    let mut beams: Vec<StructuralBeam> = Vec::new();
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
    beams: &mut Vec<StructuralBeam>,
) -> Result<Vec<StructuralBeam>, AISCError> {
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
    beams: &mut Vec<StructuralBeam>,
) -> Result<Vec<StructuralBeam>, AISCError> {
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
    use shapes::aisc_shapes::{MissingPropertyError, StructuralBeam as SB};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub StructuralBeamRepo {}

        impl ShapeRepository<SB> for StructuralBeamRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<SB>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<SB>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<SB>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<SB>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<SB>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn s12x31_8() -> SB {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("S12X31.8"))
            .aisc_manual_label(String::from("S12X31.8"))
            .w_upper(31.8)
            .a_upper(9.31)
            .d_lower(12.0)
            .ddet(12.0)
            .bf(5.0)
            .bfdet(5.0)
            .tw(0.35)
            .twdet(0.375)
            .twdet_2(0.1875)
            .tf(0.544)
            .tfdet(0.5625)
            .kdes(1.19)
            .kdet(1.1875)
            .bf_2tf(4.6)
            .h_tw(28.3)
            .ix(217.0)
            .zx(41.8)
            .sx(36.2)
            .rx(4.83)
            .iy(9.33)
            .zy(6.44)
            .sy(3.73)
            .ry(1.00)
            .j_upper(8.78)
            .cw(306.0)
            .wno(14.3)
            .sw1(9.74)
            .qf(7.24)
            .qw(20.8)
            .rts(1.21)
            .ho(11.5)
            .pa(36.3)
            .pb(41.3)
            .pc(29.1)
            .pd(34.2)
            .t(9.625)
            .wgi(3.0)
            .try_build::<SB>()
            .unwrap()
    }

    fn s12x35() -> SB {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("S12X35"))
            .aisc_manual_label(String::from("S12X35"))
            .w_upper(35.0)
            .a_upper(10.3)
            .d_lower(12.0)
            .ddet(12.0)
            .bf(5.078)
            .bfdet(5.125)
            .tw(0.428)
            .twdet(0.4375)
            .twdet_2(0.21875)
            .tf(0.544)
            .tfdet(0.5625)
            .kdes(1.19)
            .kdet(1.1875)
            .bf_2tf(4.67)
            .h_tw(23.6)
            .ix(229.0)
            .zx(45.1)
            .sx(38.2)
            .rx(4.72)
            .iy(9.87)
            .zy(6.79)
            .sy(3.89)
            .ry(0.98)
            .j_upper(0.706)
            .cw(316.0)
            .wno(14.4)
            .sw1(9.95)
            .qf(7.38)
            .qw(21.7)
            .rts(1.22)
            .ho(11.5)
            .pa(36.4)
            .pb(41.4)
            .pc(29.2)
            .pd(34.2)
            .t(9.625)
            .try_build::<SB>()
            .unwrap()
    }

    fn s10x35() -> SB {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("S10X35"))
            .aisc_manual_label(String::from("S10X35"))
            .w_upper(35.0)
            .a_upper(10.3)
            .d_lower(10.0)
            .ddet(10.0)
            .bf(4.944)
            .bfdet(5.0)
            .tw(0.594)
            .twdet(0.5625)
            .twdet_2(0.28125)
            .tf(0.491)
            .tfdet(0.5)
            .kdes(1.12)
            .kdet(1.125)
            .bf_2tf(5.03)
            .h_tw(13.1)
            .ix(147.0)
            .zx(34.2)
            .sx(29.4)
            .rx(3.78)
            .iy(8.36)
            .zy(5.74)
            .sy(3.38)
            .ry(0.901)
            .j_upper(0.541)
            .cw(186.0)
            .wno(12.3)
            .sw1(6.92)
            .qf(6.26)
            .qw(16.9)
            .rts(1.13)
            .ho(9.51)
            .pa(32.2)
            .pb(36.8)
            .pc(25.8)
            .pd(30.3)
            .t(7.875)
            .wgi(2.75)
            .try_build::<SB>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![s12x31_8(), s12x35(), s10x35()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-beams").await;

        response.assert_status_ok();
        let beams: Vec<StructuralBeam> = response.json::<Vec<StructuralBeam>>();
        assert_eq!(3, beams.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("S12X31.8")))
            .returning(|_| Box::pin(async { Ok(Some(s12x31_8())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?aisc_manual_label=S12X31.8")
            .await;

        response.assert_status_ok();
        let beams: Vec<StructuralBeam> = response.json::<Vec<StructuralBeam>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("S12X31.8"),
            beams.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("S12X31.8")))
            .returning(|_| Box::pin(async { Ok(Some(s12x31_8())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?edi_std_nomenclature=S12X31.8")
            .await;

        response.assert_status_ok();
        let beams: Vec<StructuralBeam> = response.json::<Vec<StructuralBeam>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("S12X31.8"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(12.0))
            .returning(|_| Box::pin(async { Ok(vec![s12x31_8(), s12x35()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-beams?detailing_depth=12.0").await;

        response.assert_status_ok();
        let beams: Vec<StructuralBeam> = response.json::<Vec<StructuralBeam>>();

        assert_eq!(2, beams.iter().count());
        beams.iter().for_each(|b| assert_eq!(12.0, b.ddet));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(5.0))
            .returning(|_| Box::pin(async { Ok(vec![s12x31_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?detailing_flange_width=5.0")
            .await;

        response.assert_status_ok();
        let beams: Vec<StructuralBeam> = response.json::<Vec<StructuralBeam>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("S12X31.8"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(5.0, beams.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockStructuralBeamRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(12.0))
            .returning(|_| Box::pin(async { Ok(vec![s12x31_8(), s12x35()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(5.0))
            .returning(|_| Box::pin(async { Ok(vec![s12x31_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?detailing_depth=12.0&detailing_flange_width=5.0")
            .await;

        response.assert_status_ok();
        let beams: Vec<StructuralBeam> = response.json::<Vec<StructuralBeam>>();

        assert_eq!(1, beams.iter().count());
        beams.iter().for_each(|b| {
            assert_eq!(12.0, b.ddet);
            assert_eq!(5.0, b.bfdet);
        });
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockStructuralBeamRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(4.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(5.0))
            .returning(|_| Box::pin(async { Ok(vec![s12x31_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?detailing_depth=4.0&detailing_flange_width=5.0")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-beams").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("Ix"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-beams").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("S6X12.5")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?aisc_manual_label=S6X12.5")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("S12X31.8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?aisc_manual_label=S12X31.8")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("S6X12.5")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?edi_std_nomenclature=S6X12.5")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("S12X31.8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?edi_std_nomenclature=S12X31.8")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(5.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/structural-beams?detailing_flange_width=5.0")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockStructuralBeamRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(12.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/structural-beams", get(get_structural_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/structural-beams?detailing_depth=12.0").await;
        response.assert_status_internal_server_error();
    }
}
