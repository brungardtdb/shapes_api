use crate::dto;
use crate::dto::aisc_shapes::DoubleAngle;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::DoubleAngle as DE;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Double Angle Handler
pub struct AppStateDyn {
    /// Repository for AISC cee channels
    pub repo: Arc<dyn ShapeRepository<DE>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC double angles
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

/// Gets all AISC double angles
#[debug_handler]
pub async fn get_double_angles(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    let result = state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.is_empty() {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<DoubleAngle> = shapes.iter().map(|d| d.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(d)) => {
                let shape: dto::aisc_shapes::DoubleAngle = d.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }

    if let Some(std_nom) = params.edi_std_nomenclature.clone() {
        let shape_result = &state.repo.shape_with_edi_std_nomenclature(std_nom).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(d)) => {
                let shape: dto::aisc_shapes::DoubleAngle = d.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }

    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<DoubleAngle>>, AISCError> {
    let mut double_angles: Vec<DoubleAngle> = Vec::new();
    if let Some(shorter_leg) = params.short_leg_width {
        double_angles = get_from_shorter_leg(&state, shorter_leg, &mut double_angles).await?;
        if double_angles.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(longer_leg) = params.long_leg_width {
        double_angles = get_from_longer_leg(&state, longer_leg, &mut double_angles).await?;
        if double_angles.is_empty() {
            return Err(AISCError::ShapeNotFound);
        }
    }
    Ok(AppJson(double_angles))
}

async fn get_from_shorter_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    double_angles: &mut Vec<DoubleAngle>,
) -> Result<Vec<DoubleAngle>, AISCError> {
    if double_angles.iter().nth(0).is_some() {
        return Ok(double_angles
            .iter()
            .filter(|d| d.d_lower == leg)
            .map(|d| d.clone())
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

async fn get_from_longer_leg(
    state: &Arc<AppStateDyn>,
    leg: f64,
    double_angles: &mut Vec<DoubleAngle>,
) -> Result<Vec<DoubleAngle>, AISCError> {
    if double_angles.iter().nth(0).is_some() {
        return Ok(double_angles
            .iter()
            .filter(|d| d.b_lower == leg)
            .map(|d| d.clone())
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
    use shapes::aisc_shapes::{DoubleAngle as DE, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub DoubleAngleRepo {}

        impl ShapeRepository<DE> for DoubleAngleRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<DE>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<DE>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<DE>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<DE>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<DE>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async {
                Ok(vec![
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("2L8X6X1LLBB"))
                        .with_aisc_manual_label(String::from("2L8X6X1LLBB"))
                        .with_w_upper(88.4)
                        .with_a_upper(26.2)
                        .with_d_lower(8.0)
                        .with_b_lower(6.0)
                        .with_t_lower(1.0)
                        .with_y_lower(2.65)
                        .with_yp(1.45)
                        .with_b_t(8.0)
                        .with_ix(162.0)
                        .with_zx(54.6)
                        .with_sx(30.2)
                        .with_rx(2.49)
                        .with_iy(150.0)
                        .with_zy(43.2)
                        .with_sy(25.0)
                        .with_ry(2.39)
                        .with_ro(4.06)
                        .with_h_upper(0.721)
                        .try_build::<DE>()
                        .unwrap(),
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("2L10X10X1-3/8"))
                        .with_aisc_manual_label(String::from("2L10X10X1-3/8"))
                        .with_w_upper(174.0)
                        .with_a_upper(51.2)
                        .with_d_lower(10.0)
                        .with_b_lower(10.0)
                        .with_t_lower(1.38)
                        .with_y_lower(3.0)
                        .with_yp(1.28)
                        .with_b_t(7.25)
                        .with_ix(462.0)
                        .with_zx(120.0)
                        .with_sx(66.0)
                        .with_rx(3.0)
                        .with_iy(923.0)
                        .with_zy(154.0)
                        .with_sy(92.3)
                        .with_ry(4.25)
                        .with_ro(5.69)
                        .with_h_upper(0.835)
                        .try_build::<DE>()
                        .unwrap(),
                    ShapeBuilder::new()
                        .with_edi_std_nomenclature(String::from("2L12X12X1-3/8"))
                        .with_aisc_manual_label(String::from("2L12X12X1-3/8"))
                        .with_w_upper(210.0)
                        .with_a_upper(62.2)
                        .with_d_lower(12.0)
                        .with_b_lower(12.0)
                        .with_t_lower(1.38)
                        .with_y_lower(3.5)
                        .with_yp(1.3)
                        .with_b_t(8.7)
                        .with_ix(826.0)
                        .with_zx(176.0)
                        .with_sx(97.2)
                        .with_rx(3.64)
                        .with_iy(1590.0)
                        .with_zy(218.0)
                        .with_sy(133.0)
                        .with_ry(5.06)
                        .with_ro(6.84)
                        .with_h_upper(0.831)
                        .try_build::<DE>()
                        .unwrap(),
                ])
            })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles").await;

        response.assert_status_ok();
        let double_angles: Vec<DoubleAngle> = response.json::<Vec<DoubleAngle>>();
        assert_eq!(3, double_angles.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("2L10X10X1-3/8")))
            .returning(|_| {
                Box::pin(async {
                    Ok(Some(
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L10X10X1-3/8"))
                            .with_aisc_manual_label(String::from("2L10X10X1-3/8"))
                            .with_w_upper(174.0)
                            .with_a_upper(51.2)
                            .with_d_lower(10.0)
                            .with_b_lower(10.0)
                            .with_t_lower(1.38)
                            .with_y_lower(3.0)
                            .with_yp(1.28)
                            .with_b_t(7.25)
                            .with_ix(462.0)
                            .with_zx(120.0)
                            .with_sx(66.0)
                            .with_rx(3.0)
                            .with_iy(923.0)
                            .with_zy(154.0)
                            .with_sy(92.3)
                            .with_ry(4.25)
                            .with_ro(5.69)
                            .with_h_upper(0.835)
                            .try_build::<DE>()
                            .unwrap(),
                    ))
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?aisc_manual_label=2L10X10X1-3/8")
            .await;

        response.assert_status_ok();
        let double_angles: Vec<DoubleAngle> = response.json::<Vec<DoubleAngle>>();

        assert_eq!(1, double_angles.iter().count());
        assert_eq!(
            String::from("2L10X10X1-3/8"),
            double_angles.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("2L10X10X1-3/8")))
            .returning(|_| {
                Box::pin(async {
                    Ok(Some(
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L10X10X1-3/8"))
                            .with_aisc_manual_label(String::from("2L10X10X1-3/8"))
                            .with_w_upper(174.0)
                            .with_a_upper(51.2)
                            .with_d_lower(10.0)
                            .with_b_lower(10.0)
                            .with_t_lower(1.38)
                            .with_y_lower(3.0)
                            .with_yp(1.28)
                            .with_b_t(7.25)
                            .with_ix(462.0)
                            .with_zx(120.0)
                            .with_sx(66.0)
                            .with_rx(3.0)
                            .with_iy(923.0)
                            .with_zy(154.0)
                            .with_sy(92.3)
                            .with_ry(4.25)
                            .with_ro(5.69)
                            .with_h_upper(0.835)
                            .try_build::<DE>()
                            .unwrap(),
                    ))
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?edi_std_nomenclature=2L10X10X1-3/8")
            .await;

        response.assert_status_ok();
        let double_angles: Vec<DoubleAngle> = response.json::<Vec<DoubleAngle>>();

        assert_eq!(1, double_angles.iter().count());
        assert_eq!(
            String::from("2L10X10X1-3/8"),
            double_angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L8X6X1LLBB"))
                            .with_aisc_manual_label(String::from("2L8X6X1LLBB"))
                            .with_w_upper(88.4)
                            .with_a_upper(26.2)
                            .with_d_lower(8.0)
                            .with_b_lower(6.0)
                            .with_t_lower(1.0)
                            .with_y_lower(2.65)
                            .with_yp(1.45)
                            .with_b_t(8.0)
                            .with_ix(162.0)
                            .with_zx(54.6)
                            .with_sx(30.2)
                            .with_rx(2.49)
                            .with_iy(150.0)
                            .with_zy(43.2)
                            .with_sy(25.0)
                            .with_ry(2.39)
                            .with_ro(4.06)
                            .with_h_upper(0.721)
                            .try_build::<DE>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles?short_leg_width=8.0").await;

        response.assert_status_ok();
        let double_angles: Vec<DoubleAngle> = response.json::<Vec<DoubleAngle>>();

        assert_eq!(1, double_angles.iter().count());
        assert_eq!(
            String::from("2L8X6X1LLBB"),
            double_angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(8.0, double_angles.iter().nth(0).unwrap().d_lower);
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(12.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L12X12X1-3/8"))
                            .with_aisc_manual_label(String::from("2L12X12X1-3/8"))
                            .with_w_upper(210.0)
                            .with_a_upper(62.2)
                            .with_d_lower(12.0)
                            .with_b_lower(12.0)
                            .with_t_lower(1.38)
                            .with_y_lower(3.5)
                            .with_yp(1.3)
                            .with_b_t(8.7)
                            .with_ix(826.0)
                            .with_zx(176.0)
                            .with_sx(97.2)
                            .with_rx(3.64)
                            .with_iy(1590.0)
                            .with_zy(218.0)
                            .with_sy(133.0)
                            .with_ry(5.06)
                            .with_ro(6.84)
                            .with_h_upper(0.831)
                            .try_build::<DE>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles?long_leg_width=12.0").await;

        response.assert_status_ok();
        let double_angles: Vec<DoubleAngle> = response.json::<Vec<DoubleAngle>>();

        assert_eq!(1, double_angles.iter().count());
        assert_eq!(
            String::from("2L12X12X1-3/8"),
            double_angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(12.0, double_angles.iter().nth(0).unwrap().b_lower);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L8X6X1LLBB"))
                            .with_aisc_manual_label(String::from("2L8X6X1LLBB"))
                            .with_w_upper(88.4)
                            .with_a_upper(26.2)
                            .with_d_lower(8.0)
                            .with_b_lower(6.0)
                            .with_t_lower(1.0)
                            .with_y_lower(2.65)
                            .with_yp(1.45)
                            .with_b_t(8.0)
                            .with_ix(162.0)
                            .with_zx(54.6)
                            .with_sx(30.2)
                            .with_rx(2.49)
                            .with_iy(150.0)
                            .with_zy(43.2)
                            .with_sy(25.0)
                            .with_ry(2.39)
                            .with_ro(4.06)
                            .with_h_upper(0.721)
                            .try_build::<DE>()
                            .unwrap(),
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L8X8X1"))
                            .with_aisc_manual_label(String::from("2L8X8X1"))
                            .with_w_upper(102.0)
                            .with_a_upper(30.2)
                            .with_d_lower(8.0)
                            .with_b_lower(8.0)
                            .with_t_lower(1.0)
                            .with_y_lower(2.36)
                            .with_yp(0.944)
                            .with_b_t(8.0)
                            .with_ix(178.0)
                            .with_zx(57.0)
                            .with_sx(31.6)
                            .with_rx(2.43)
                            .with_iy(347.0)
                            .with_zy(71.3)
                            .with_sy(43.4)
                            .with_ry(3.39)
                            .with_ro(4.56)
                            .with_h_upper(0.834)
                            .try_build::<DE>()
                            .unwrap(),
                    ])
                })
            });

        repo.expect_shapes_with_width()
            .with(predicate::eq(6.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L8X6X1LLBB"))
                            .with_aisc_manual_label(String::from("2L8X6X1LLBB"))
                            .with_w_upper(88.4)
                            .with_a_upper(26.2)
                            .with_d_lower(8.0)
                            .with_b_lower(6.0)
                            .with_t_lower(1.0)
                            .with_y_lower(2.65)
                            .with_yp(1.45)
                            .with_b_t(8.0)
                            .with_ix(162.0)
                            .with_zx(54.6)
                            .with_sx(30.2)
                            .with_rx(2.49)
                            .with_iy(150.0)
                            .with_zy(43.2)
                            .with_sy(25.0)
                            .with_ry(2.39)
                            .with_ro(4.06)
                            .with_h_upper(0.721)
                            .try_build::<DE>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?short_leg_width=8.0&long_leg_width=6.0")
            .await;

        response.assert_status_ok();
        let double_angles: Vec<DoubleAngle> = response.json::<Vec<DoubleAngle>>();

        assert_eq!(1, double_angles.iter().count());
        assert_eq!(
            String::from("2L8X6X1LLBB"),
            double_angles.iter().nth(0).unwrap().edi_std_nomenclature
        );
        assert_eq!(8.0, double_angles.iter().nth(0).unwrap().d_lower);
        assert_eq!(6.0, double_angles.iter().nth(0).unwrap().b_lower);
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(4.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(10.0))
            .returning(|_| {
                Box::pin(async {
                    Ok(vec![
                        ShapeBuilder::new()
                            .with_edi_std_nomenclature(String::from("2L10X10X1-3/8"))
                            .with_aisc_manual_label(String::from("2L10X10X1-3/8"))
                            .with_w_upper(174.0)
                            .with_a_upper(51.2)
                            .with_d_lower(10.0)
                            .with_b_lower(10.0)
                            .with_t_lower(1.38)
                            .with_y_lower(3.0)
                            .with_yp(1.28)
                            .with_b_t(7.25)
                            .with_ix(462.0)
                            .with_zx(120.0)
                            .with_sx(66.0)
                            .with_rx(3.0)
                            .with_iy(923.0)
                            .with_zy(154.0)
                            .with_sy(92.3)
                            .with_ry(4.25)
                            .with_ro(5.69)
                            .with_h_upper(0.835)
                            .try_build::<DE>()
                            .unwrap(),
                    ])
                })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?short_leg_width=4.0&long_leg_width=10.0")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_all().returning(|| {
            Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
        });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("2L8X6X1LLBB")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?aisc_manual_label=2L8X6X1LLBB")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("2L10X10X1-3/8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?aisc_manual_label=2L10X10X1-3/8")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("2L8X6X1LLBB")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?edi_std_nomenclature=2L8X6X1LLBB")
            .await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("2L10X10X1-3/8")))
            .returning(|_| {
                Box::pin(async { Err(MissingPropertyError::from("AISC Manual Label"))? })
            });

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/double-angles?edi_std_nomenclature=2L10X10X1-3/8")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(6.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("b"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles?long_leg_width=6.0").await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockDoubleAngleRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(8.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("d"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/double-angles", get(get_double_angles))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/double-angles?short_leg_width=8.0").await;
        response.assert_status_internal_server_error();
    }
}
