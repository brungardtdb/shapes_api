use crate::dto;
use crate::dto::aisc_shapes::Angle;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::Angle as A;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Angle Handler
pub struct AppStateDyn {
    /// Repository for AISC angles
    pub repo: Arc<dyn ShapeRepository<A>>,
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
    pub long_leg_width: Option<f64>,
    /// Beam Width
    pub short_leg_width: Option<f64>,
}

/// Gets all AISC angles
#[debug_handler]
pub async fn get_angles(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<Angle>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.is_empty() {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<Angle> = shapes.iter().map(|s| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::Angle = s.into();
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
                let shape: dto::aisc_shapes::Angle = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<Angle>>, AISCError> {
    let mut angles: Vec<Angle> = Vec::new();
    if let Some(shorter_leg) = params.short_leg_width {
        angles = get_from_shorter_leg(&state, shorter_leg, &mut angles).await?;
        if *&angles.is_empty() {
            // return early because we cannot meet additional search criteria
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(longer_leg) = params.long_leg_width {
        angles = get_from_longer_leg(&state, longer_leg, &mut angles).await?;
        if *&angles.is_empty() {
            // return early because we cannot meet additional search criteria
            return Err(AISCError::ShapeNotFound);
        }
    }
    Ok(AppJson(angles))
}

async fn get_from_shorter_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    angles: &mut Vec<Angle>,
) -> Result<Vec<Angle>, AISCError> {
    if angles.iter().nth(0).is_some() {
        return Ok(angles
            .iter()
            .filter(|a| a.d_lower == leg)
            .map(|a| a.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_width(leg).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => {
            return Ok(shapes.iter().map(|s| s.into()).collect::<Vec<_>>());
        }
    }
}

async fn get_from_longer_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    angles: &mut Vec<Angle>,
) -> Result<Vec<Angle>, AISCError> {
    if angles.iter().nth(0).is_some() {
        return Ok(angles
            .iter()
            .filter(|a| a.b_lower == leg)
            .map(|a| a.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_depth(leg).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(shapes) => {
            return Ok(shapes.iter().map(|s| s.into()).collect::<Vec<_>>());
        }
    }
}

fn has_query(params: &Params) -> bool {
    if let Some(_) = params.aisc_manual_label {
        return true;
    }
    if let Some(_) = params.edi_std_nomenclature {
        return true;
    }
    if let Some(_) = params.long_leg_width {
        return true;
    }
    if let Some(_) = params.short_leg_width {
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
    use shapes::aisc_shapes::{Angle as A, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub AngleRepo {}

        impl ShapeRepository<A> for AngleRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<A>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<A>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<A>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<A>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<A>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn l6x4x3_8() -> A {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("L6X4X3/8"))
            .aisc_manual_label(String::from("L6X4X3/8"))
            .w_upper(12.3)
            .a_upper(3.61)
            .d_lower(4.0)
            .b_lower(6.0)
            .t_lower(0.375)
            .kdes(0.875)
            .kdet(0.875)
            .x_lower(0.933)
            .y_lower(1.93)
            .xp(0.301)
            .yp(1.19)
            .b_t(16.0)
            .ix(13.4)
            .zx(5.89)
            .sx(3.30)
            .rx(1.93)
            .iy(4.86)
            .zy(2.79)
            .sy(1.58)
            .ry(1.16)
            .iz(2.73)
            .rz(0.870)
            .sz(1.31)
            .j_upper(0.177)
            .cw(0.369)
            .ro(2.94)
            .tan_a(0.446)
            .iw(15.5)
            .za(2.84)
            .zb(1.38)
            .zc(4.02)
            .wa(2.09)
            .wb(1.64)
            .wc(0.979)
            .swa(5.46)
            .swb(11.2)
            .swc(3.85)
            .sza(1.31)
            .szb(1.66)
            .szc(2.79)
            .pa(16.0)
            .pa_2(14.0)
            .pb(20.0)
            .try_build::<A>()
            .unwrap()
    }

    fn l8x6x1_2() -> A {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("L8X6X1/2"))
            .aisc_manual_label(String::from("L8X6X1/2"))
            .w_upper(23.0)
            .a_upper(6.8)
            .d_lower(6.0)
            .b_lower(8.0)
            .t_lower(0.5)
            .kdes(1.0)
            .kdet(1.0)
            .x_lower(1.46)
            .y_lower(2.46)
            .xp(0.425)
            .yp(1.2)
            .b_t(16.0)
            .ix(44.4)
            .zx(14.6)
            .sx(8.01)
            .rx(2.55)
            .iy(21.7)
            .zy(8.52)
            .sy(4.79)
            .ry(1.79)
            .iz(11.5)
            .rz(1.30)
            .sz(3.98)
            .j_upper(0.584)
            .cw(2.28)
            .ro(4.01)
            .tan_a(0.557)
            .iw(54.6)
            .za(4.14)
            .zb(1.44)
            .zc(5.41)
            .wa(2.87)
            .wb(2.51)
            .wc(1.62)
            .swa(14.8)
            .swb(42.2)
            .swc(11.2)
            .sza(4.46)
            .szb(5.1)
            .szc(7.9)
            .pa(22.0)
            .pa_2(20.0)
            .pb(28.0)
            .try_build::<A>()
            .unwrap()
    }

    fn l8x8x1() -> A {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("L8X8X1"))
            .aisc_manual_label(String::from("L8X8X1"))
            .w_upper(51.0)
            .a_upper(15.1)
            .d_lower(8.0)
            .b_lower(8.0)
            .t_lower(1.0)
            .kdes(1.63)
            .kdet(1.625)
            .x_lower(2.36)
            .y_lower(2.36)
            .xp(0.944)
            .yp(0.944)
            .b_t(8.0)
            .ix(89.1)
            .zx(28.5)
            .sx(15.8)
            .rx(2.43)
            .iy(89.1)
            .zy(28.5)
            .sy(15.8)
            .ry(2.43)
            .iz(36.8)
            .rz(1.56)
            .sz(11.0)
            .j_upper(5.08)
            .cw(23.4)
            .ro(4.32)
            .h_upper(0.63)
            .tan_a(1.0)
            .iw(1.41)
            .za(5.30)
            .zb(0.0)
            .zc(5.3)
            .wa(2.67)
            .wb(3.34)
            .wc(2.67)
            .swa(26.6)
            .swc(26.6)
            .sza(13.8)
            .szb(11.0)
            .szc(13.8)
            .pa(24.0)
            .pa_2(24.0)
            .pb(32.0)
            .try_build::<A>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockAngleRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Ok(vec![l6x4x3_8(), l8x6x1_2(), l8x8x1()]) })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles").await;

        response.assert_status_ok();
        let angles: Vec<Angle> = response.json::<Vec<Angle>>();
        assert_eq!(3, angles.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("L6X4X3/8")))
            .returning(|_| Box::pin(async { Ok(Some(l6x4x3_8())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?aisc_manual_label=L6X4X3/8").await;

        response.assert_status_ok();
        let angles: Vec<Angle> = response.json::<Vec<Angle>>();

        assert_eq!(1, *&angles.iter().count());
        assert_eq!(
            String::from("L6X4X3/8"),
            angles.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("L6X4X3/8")))
            .returning(|_| Box::pin(async { Ok(Some(l6x4x3_8())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?edi_std_nomenclature=L6X4X3/8").await;

        response.assert_status_ok();
        let angles: Vec<Angle> = response.json::<Vec<Angle>>();

        assert_eq!(1, *&angles.iter().count());
        assert_eq!(
            String::from("L6X4X3/8"),
            angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.0))
            .returning(|_| Box::pin(async { Ok(vec![l6x4x3_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?long_leg_width=6.0").await;

        response.assert_status_ok();
        let angles: Vec<Angle> = response.json::<Vec<Angle>>();

        assert_eq!(1, *&angles.iter().count());
        assert_eq!(
            String::from("L6X4X3/8"),
            *&angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(6.0, angles.iter().nth(0).unwrap().b_lower);
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(4.0))
            .returning(|_| Box::pin(async { Ok(vec![l6x4x3_8()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?short_leg_width=4.0").await;

        response.assert_status_ok();
        let angles: Vec<Angle> = response.json::<Vec<Angle>>();

        assert_eq!(1, *&angles.iter().count());
        assert_eq!(
            String::from("L6X4X3/8"),
            *&angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(4.0, angles.iter().nth(0).unwrap().d_lower);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(6.0))
            .returning(|_| Box::pin(async { Ok(vec![l8x6x1_2()]) }));

        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0))
            .returning(|_| Box::pin(async { Ok(vec![l8x6x1_2(), l8x8x1()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/angles?long_leg_width=8.0&short_leg_width=6.0")
            .await;

        response.assert_status_ok();
        let angles: Vec<Angle> = response.json::<Vec<Angle>>();

        assert_eq!(1, *&angles.iter().count());
        assert_eq!(
            String::from("L8X6X1/2"),
            *&angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(8.0, *&angles.clone().iter().nth(0).unwrap().b_lower);
        assert_eq!(6.0, angles.clone().iter().nth(0).unwrap().d_lower);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(4.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0))
            .returning(|_| Box::pin(async { Ok(vec![l8x6x1_2(), l8x8x1()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/angles?long_leg_width=8.0&short_leg_width=4.0")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockAngleRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockAngleRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("L8X8X1")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?aisc_manual_label=L8X8X1").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("L6X4X3/8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?aisc_manual_label=L6X4X3/8").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("L6X4X3/8")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?edi_std_nomenclature=L6X4X3/8").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("L6X4X3/8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?edi_std_nomenclature=L6X4X3/8").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(6.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("d"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?short_leg_width=6.0").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockAngleRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(6.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("b"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/angles", get(get_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/angles?long_leg_width=6.0").await;
        response.assert_status_internal_server_error();
    }
}
