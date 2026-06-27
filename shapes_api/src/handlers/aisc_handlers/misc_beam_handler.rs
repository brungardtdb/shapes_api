use crate::dto;
use crate::dto::aisc_shapes::MiscBeam;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::MiscBeam as MB;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};
/// Dynamic App State for Misc Beam Handler
pub struct AppStateDyn {
    /// Repository for misc beam shapes
    pub repo: Arc<dyn ShapeRepository<MB>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC misc beam
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

/// Gets all misc beam AISC shapes
#[debug_handler]
pub async fn get_misc_beams(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<MiscBeam>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<MiscBeam>>, AISCError> {
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

            let result: Vec<MiscBeam> = shapes.iter().map(|s: &MB| s.into()).collect::<Vec<_>>();
            return Ok(AppJson(result));
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<MiscBeam>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::MiscBeam = s.into();
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
                let shape: dto::aisc_shapes::MiscBeam = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<MiscBeam>>, AISCError> {
    let mut beams: Vec<MiscBeam> = Vec::new();
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
    beams: &mut Vec<MiscBeam>,
) -> Result<Vec<MiscBeam>, AISCError> {
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
    beams: &mut Vec<MiscBeam>,
) -> Result<Vec<MiscBeam>, AISCError> {
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
    use shapes::aisc_shapes::{MiscBeam as MB, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub MiscBeamRepo {}

        impl ShapeRepository<MB> for MiscBeamRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<MB>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<MB>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<MB>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<MB>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<MB>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn m12_5x12_4() -> MB {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("M12.5X12.4"))
            .aisc_manual_label(String::from("M12.5X12.4"))
            .t_f(false)
            .w_upper(12.4)
            .a_upper(3.63)
            .d_lower(12.5)
            .ddet(12.5)
            .bf(3.75)
            .bfdet(3.75)
            .tw(0.155)
            .twdet(0.125)
            .twdet_2(0.0625)
            .tf(0.228)
            .tfdet(0.25)
            .kdes(0.563)
            .kdet(0.5625)
            .k1(0.375)
            .bf_2tf(8.22)
            .h_tw(74.8)
            .ix(89.3)
            .zx(16.5)
            .sx(14.2)
            .rx(4.96)
            .iy(2.01)
            .zy(1.68)
            .sy(1.07)
            .ry(0.744)
            .j_upper(0.0493)
            .cw(76.0)
            .wno(11.5)
            .sw1(2.46)
            .qf(2.51)
            .qw(8.06)
            .rts(0.933)
            .ho(12.3)
            .pa(35.5)
            .pb(39.3)
            .pc(28.8)
            .pd(32.5)
            .t(11.375)
            .try_build::<MB>()
            .unwrap()
    }

    fn m12_5x11_6() -> MB {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("M12.5X11.6"))
            .aisc_manual_label(String::from("M12.5X11.6"))
            .t_f(false)
            .w_upper(11.6)
            .a_upper(3.4)
            .d_lower(12.5)
            .ddet(12.5)
            .bf(3.5)
            .bfdet(3.5)
            .tw(0.155)
            .twdet(0.125)
            .twdet_2(0.0625)
            .tf(0.211)
            .tfdet(0.1875)
            .kdes(0.563)
            .kdet(0.5625)
            .k1(0.375)
            .bf_2tf(8.29)
            .h_tw(74.8)
            .ix(80.3)
            .zx(15.0)
            .sx(12.8)
            .rx(4.86)
            .iy(1.51)
            .zy(1.37)
            .sy(0.864)
            .ry(0.667)
            .j_upper(0.0414)
            .cw(57.1)
            .wno(10.8)
            .sw1(1.99)
            .qf(2.17)
            .qw(7.36)
            .rts(0.852)
            .ho(12.3)
            .pa(34.8)
            .pb(38.3)
            .pc(28.5)
            .pd(32.0)
            .t(11.375)
            .try_build::<MB>()
            .unwrap()
    }

    fn m12x11_8() -> MB {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("M12X11.8"))
            .aisc_manual_label(String::from("M12X11.8"))
            .t_f(false)
            .w_upper(11.8)
            .a_upper(3.47)
            .d_lower(12.0)
            .ddet(12.0)
            .bf(3.07)
            .bfdet(3.125)
            .tw(0.177)
            .twdet(0.1875)
            .twdet_2(0.125)
            .tf(0.225)
            .tfdet(0.25)
            .kdes(0.563)
            .kdet(0.5625)
            .k1(0.375)
            .bf_2tf(6.81)
            .h_tw(62.5)
            .ix(72.2)
            .zx(14.3)
            .sx(12.0)
            .rx(4.56)
            .iy(1.09)
            .zy(1.15)
            .sy(0.709)
            .ry(0.559)
            .j_upper(0.05)
            .cw(37.7)
            .wno(9.04)
            .sw1(1.56)
            .qf(1.92)
            .qw(7.02)
            .rts(0.731)
            .ho(11.8)
            .pa(32.4)
            .pb(35.5)
            .pc(27.1)
            .pd(30.1)
            .t(10.875)
            .try_build::<MB>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![m12_5x12_4(), m12_5x11_6(), m12x11_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams").await;

        response.assert_status_ok();
        let beams: Vec<MiscBeam> = response.json::<Vec<MiscBeam>>();
        assert_eq!(3, beams.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("M12X11.8")))
            .returning(|_| Box::pin(async { Ok(Some(m12x11_8())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?aisc_manual_label=M12X11.8").await;

        response.assert_status_ok();
        let beams: Vec<MiscBeam> = response.json::<Vec<MiscBeam>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("M12X11.8"),
            beams.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("M12X11.8")))
            .returning(|_| Box::pin(async { Ok(Some(m12x11_8())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-beams?edi_std_nomenclature=M12X11.8")
            .await;

        response.assert_status_ok();
        let beams: Vec<MiscBeam> = response.json::<Vec<MiscBeam>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("M12X11.8"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(12.5))
            .returning(|_| Box::pin(async { Ok(vec![m12_5x12_4(), m12_5x11_6()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?detailing_depth=12.5").await;

        response.assert_status_ok();
        let beams: Vec<MiscBeam> = response.json::<Vec<MiscBeam>>();

        assert_eq!(2, beams.iter().count());
        beams.iter().for_each(|b| assert_eq!(12.5, b.ddet));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(3.5))
            .returning(|_| Box::pin(async { Ok(vec![m12_5x11_6()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?detailing_flange_width=3.5").await;

        response.assert_status_ok();
        let beams: Vec<MiscBeam> = response.json::<Vec<MiscBeam>>();

        assert_eq!(1, beams.iter().count());
        assert_eq!(
            String::from("M12.5X11.6"),
            beams.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(3.5, beams.iter().nth(0).unwrap().bfdet);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockMiscBeamRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(12.5))
            .returning(|_| Box::pin(async { Ok(vec![m12_5x12_4(), m12_5x11_6()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(3.5))
            .returning(|_| Box::pin(async { Ok(vec![m12_5x11_6()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-beams?detailing_depth=12.5&detailing_flange_width=3.5")
            .await;

        response.assert_status_ok();
        let beams: Vec<MiscBeam> = response.json::<Vec<MiscBeam>>();

        assert_eq!(1, beams.iter().count());
        beams.iter().for_each(|b| {
            assert_eq!(12.5, b.ddet);
            assert_eq!(3.5, b.bfdet);
        });
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockMiscBeamRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(4.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(3.5))
            .returning(|_| Box::pin(async { Ok(vec![m12_5x11_6()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-beams?detailing_depth=4.0&detailing_flange_width=3.5")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("Ix"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("M12X10")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?aisc_manual_label=M12X10").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("M12X11.8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?aisc_manual_label=M12X11.8").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("M12X10")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?edi_std_nomenclature=M12X10").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("M12X11.8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-beams?edi_std_nomenclature=M12X11.8")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(3.5_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?detailing_flange_width=3.5").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockMiscBeamRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(12.5_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-beams", get(get_misc_beams))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-beams?detailing_depth=12.5").await;
        response.assert_status_internal_server_error();
    }
}
