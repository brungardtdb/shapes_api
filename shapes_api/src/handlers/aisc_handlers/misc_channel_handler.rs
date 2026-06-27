use crate::dto;
use crate::dto::aisc_shapes::MiscChannel;
use crate::error_handling::aisc::{AISCError, AppJson};
use axum::extract::Query;
use serde::Deserialize;
use shapes::aisc_shapes::MiscChannel as MC;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

use axum::{debug_handler, extract::State};

/// Dynamic App State for Misc Channel Handler
pub struct AppStateDyn {
    /// Repository for AISC misc channels
    pub repo: Arc<dyn ShapeRepository<MC>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Query parameters for AISC misc channel
pub struct Params {
    /// AISC Manual Label
    pub aisc_manual_label: Option<String>,
    /// EDI Std. Nomenclature
    pub edi_std_nomenclature: Option<String>,
    /// Channel Depth
    pub detailing_depth: Option<f64>,
    /// Channel Flange Width
    pub detailing_flange_width: Option<f64>,
}

/// Gets all AISC Misc Channels
#[debug_handler]
pub async fn get_misc_channels(
    State(state): State<Arc<AppStateDyn>>,
    Query(params): Query<Params>,
) -> Result<AppJson<Vec<MiscChannel>>, AISCError> {
    if has_query(&params) {
        return get_from_query(state, &params).await;
    }
    return get_all(state).await;
}

async fn get_all(state: Arc<AppStateDyn>) -> Result<AppJson<Vec<MiscChannel>>, AISCError> {
    let result = &state.repo.all().await;
    match result {
        Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
        Ok(shapes) => {
            if shapes.is_empty() {
                return Err(AISCError::DataError(Box::from(
                    "Unable to retrieve shapes from the AISC shape database".to_owned(),
                )));
            }

            let result: Vec<MiscChannel> = shapes.iter().map(|s| s.into()).collect::<Vec<_>>();
            Ok(AppJson(result))
        }
    }
}

async fn get_from_query(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<MiscChannel>>, AISCError> {
    // Check for AISC Manual Label
    if let Some(label) = params.aisc_manual_label.clone() {
        let shape_result = &state.repo.shape_with_aisc_manual_label(label).await;
        match shape_result {
            Err(err) => return Err(AISCError::DataError(Box::from(err.to_string()))),
            Ok(None) => return Err(AISCError::ShapeNotFound),
            Ok(Some(s)) => {
                let shape: dto::aisc_shapes::MiscChannel = s.into();
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
                let shape: dto::aisc_shapes::MiscChannel = s.into();
                return Ok(AppJson(vec![shape]));
            }
        }
    }
    Ok(get_from_geometry(state, params).await?)
}

async fn get_from_geometry(
    state: Arc<AppStateDyn>,
    params: &Params,
) -> Result<AppJson<Vec<MiscChannel>>, AISCError> {
    let mut channels: Vec<MiscChannel> = Vec::new();
    if let Some(depth) = params.detailing_depth {
        channels = get_from_detailing_depth(&state, depth, &mut channels).await?;
        if *&channels.is_empty() {
            // return early because we cannot meet additional search criteria
            return Err(AISCError::ShapeNotFound);
        }
    }
    if let Some(flange_width) = params.detailing_flange_width {
        channels = get_from_detailing_flange_width(&state, flange_width, &mut channels).await?;
        if *&channels.is_empty() {
            // return early because we cannot meet additional search criteria
            return Err(AISCError::ShapeNotFound);
        }
    }
    Ok(AppJson(channels))
}

async fn get_from_detailing_depth(
    state: &Arc<AppStateDyn>,
    depth: f64,
    channels: &mut Vec<MiscChannel>,
) -> Result<Vec<MiscChannel>, AISCError> {
    if channels.iter().nth(0).is_some() {
        return Ok(channels
            .iter()
            .filter(|c| c.ddet == depth)
            .map(|c| c.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_depth(depth).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(channels) => {
            return Ok(channels.iter().map(|c| c.into()).collect::<Vec<_>>());
        }
    }
}

async fn get_from_detailing_flange_width(
    state: &Arc<AppStateDyn>,
    flange_width: f64,
    channels: &mut Vec<MiscChannel>,
) -> Result<Vec<MiscChannel>, AISCError> {
    if channels.iter().nth(0).is_some() {
        return Ok(channels
            .iter()
            .filter(|c| c.bfdet == flange_width)
            .map(|c| c.clone())
            .collect::<Vec<_>>());
    }
    let result = state.repo.shapes_with_width(flange_width).await;
    match result {
        Err(err) => {
            return Err(AISCError::DataError(Box::from(err.to_string())));
        }
        Ok(channels) => {
            return Ok(channels.iter().map(|c| c.into()).collect::<Vec<_>>());
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
    use shapes::aisc_shapes::{MiscChannel as MC, MissingPropertyError};
    use std::error::Error;
    use std::future::Future;
    use std::pin::Pin;

    mock! {
        pub MiscChannelRepo {}

        impl ShapeRepository<MC> for MiscChannelRepo {
            fn all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<MC>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_aisc_manual_label<'a>(
                &'a self,
                aisc_manual_label: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<MC>, Box<dyn Error>>> + Send + 'a>>;

            fn shape_with_edi_std_nomenclature<'a>(
                &'a self,
                edi_std_nomenclature: String,
            ) -> Pin<Box<dyn Future<Output = Result<Option<MC>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_depth<'a>(
                &'a self,
                depth: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<MC>, Box<dyn Error>>> + Send + 'a>>;

            fn shapes_with_width<'a>(
                &'a self,
                width: f64,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<MC>, Box<dyn Error>>> + Send + 'a>>;
        }
    }

    fn mc18x58() -> MC {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("MC18X58"))
            .aisc_manual_label(String::from("MC18X58"))
            .w_upper(58.0)
            .a_upper(17.1)
            .d_lower(18.0)
            .ddet(18.0)
            .bf(4.2)
            .bfdet(4.25)
            .tw(0.7)
            .twdet(0.6875)
            .twdet_2(0.375)
            .tf(0.625)
            .tfdet(0.625)
            .kdes(1.44)
            .kdet(1.4375)
            .x_lower(0.862)
            .eo(0.695)
            .xp(0.474)
            .b_t(6.72)
            .h_tw(21.6)
            .ix(675.0)
            .zx(95.4)
            .sx(75.0)
            .rx(6.29)
            .iy(17.6)
            .zy(10.7)
            .sy(5.28)
            .ry(1.02)
            .j_upper(2.81)
            .cw(1070.0)
            .wno(24.4)
            .sw1(21.4)
            .sw2(18.4)
            .sw3(9.28)
            .qf(19.0)
            .qw(47.4)
            .ro(6.56)
            .h_upper(0.944)
            .rts(1.35)
            .ho(17.4)
            .pa(47.0)
            .pb(51.2)
            .pc(40.2)
            .pd(44.4)
            .t(15.125)
            .wgi(2.5)
            .try_build::<MC>()
            .unwrap()
    }

    fn mc18x51_9() -> MC {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("MC18X51.9"))
            .aisc_manual_label(String::from("MC18X51.9"))
            .w_upper(51.9)
            .a_upper(15.3)
            .d_lower(18.0)
            .ddet(18.0)
            .bf(4.1)
            .bfdet(4.125)
            .tw(0.6)
            .twdet(0.625)
            .twdet_2(0.3125)
            .tf(0.625)
            .tfdet(0.625)
            .kdes(1.44)
            .kdet(1.4375)
            .x_lower(0.858)
            .eo(0.797)
            .xp(0.424)
            .b_t(6.56)
            .h_tw(25.2)
            .ix(627.0)
            .zx(87.3)
            .sx(69.6)
            .rx(6.41)
            .iy(16.3)
            .zy(9.86)
            .sy(5.02)
            .ry(1.03)
            .j_upper(2.03)
            .cw(985.0)
            .wno(23.5)
            .sw1(19.9)
            .sw2(16.6)
            .sw3(8.31)
            .qf(19.0)
            .qw(43.3)
            .ro(6.7)
            .h_upper(0.939)
            .rts(1.35)
            .ho(17.4)
            .pa(46.7)
            .pb(50.8)
            .pc(40.1)
            .pd(44.2)
            .t(15.125)
            .wgi(2.5)
            .try_build::<MC>()
            .unwrap()
    }

    fn mc13x50() -> MC {
        ShapeBuilder::new()
            .edi_std_nomenclature(String::from("MC13X50"))
            .aisc_manual_label(String::from("MC13X50"))
            .w_upper(50.0)
            .a_upper(14.7)
            .d_lower(13.0)
            .ddet(13.0)
            .bf(4.41)
            .bfdet(4.375)
            .tw(0.787)
            .twdet(0.8125)
            .twdet_2(0.4375)
            .tf(0.61)
            .tfdet(0.625)
            .kdes(1.44)
            .kdet(1.4375)
            .x_lower(0.974)
            .eo(0.815)
            .xp(0.566)
            .b_t(7.23)
            .h_tw(13.2)
            .ix(314.0)
            .zx(60.8)
            .sx(48.3)
            .rx(4.62)
            .iy(16.4)
            .zy(10.2)
            .sy(4.77)
            .ry(1.06)
            .j_upper(2.96)
            .cw(558.0)
            .wno(17.4)
            .sw1(14.9)
            .sw2(12.1)
            .sw3(6.18)
            .qf(13.7)
            .qw(30.3)
            .ro(5.07)
            .h_upper(0.875)
            .rts(1.41)
            .ho(12.4)
            .pa(37.6)
            .pb(42.0)
            .pc(30.4)
            .pd(34.8)
            .t(10.125)
            .wgi(2.5)
            .try_build::<MC>()
            .unwrap()
    }

    #[tokio::test]
    async fn returns_all_shapes_w_no_query() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![mc18x58(), mc18x51_9(), mc13x50()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels").await;

        response.assert_status_ok();
        let channels: Vec<MiscChannel> = response.json::<Vec<MiscChannel>>();
        assert_eq!(3, channels.iter().count());
    }

    #[tokio::test]
    async fn returns_shape_w_aisc_manual_label() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("MC13X50")))
            .returning(|_| Box::pin(async { Ok(Some(mc13x50())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?aisc_manual_label=MC13X50").await;
        response.assert_status_ok();

        let channels: Vec<MiscChannel> = response.json::<Vec<MiscChannel>>();
        assert_eq!(1, *&channels.iter().count());
        assert_eq!(
            String::from("MC13X50"),
            channels.iter().nth(0).unwrap().aisc_manual_label
        );
    }

    #[tokio::test]
    async fn returns_shape_w_edi_std_nomenclature() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("MC13X50")))
            .returning(|_| Box::pin(async { Ok(Some(mc13x50())) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-channels?edi_std_nomenclature=MC13X50")
            .await;
        response.assert_status_ok();

        let channels: Vec<MiscChannel> = response.json::<Vec<MiscChannel>>();
        assert_eq!(1, *&channels.iter().count());
        assert_eq!(
            String::from("MC13X50"),
            channels.iter().nth(0).unwrap().edi_std_nomenclature
        );
    }

    #[tokio::test]
    async fn filters_on_depth() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(18.0))
            .returning(|_| Box::pin(async { Ok(vec![mc18x58(), mc18x51_9()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?detailing_depth=18.0").await;
        response.assert_status_ok();

        let channels: Vec<MiscChannel> = response.json::<Vec<MiscChannel>>();
        assert_eq!(2, *&channels.iter().count());
        channels.iter().for_each(|c| assert_eq!(18.0, c.ddet));
    }

    #[tokio::test]
    async fn filters_on_width() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(4.125))
            .returning(|_| Box::pin(async { Ok(vec![mc18x51_9()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-channels?detailing_flange_width=4.125")
            .await;
        response.assert_status_ok();

        let channels: Vec<MiscChannel> = response.json::<Vec<MiscChannel>>();
        assert_eq!(1, *&channels.iter().count());
        channels.iter().for_each(|c| assert_eq!(4.125, c.bfdet));
    }

    #[tokio::test]
    async fn filters_on_width_and_depth() {
        let mut repo = MockMiscChannelRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(18.0))
            .returning(|_| Box::pin(async { Ok(vec![mc18x58(), mc18x51_9()]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(4.25))
            .returning(|_| Box::pin(async { Ok(vec![mc18x58(), mc13x50()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-channels?detailing_depth=18.0&detailing_flange_width=4.25")
            .await;
        response.assert_status_ok();

        let channels: Vec<MiscChannel> = response.json::<Vec<MiscChannel>>();
        assert_eq!(1, *&channels.iter().count());
        channels.iter().for_each(|c| {
            assert_eq!(18.0, c.ddet);
            assert_eq!(4.25, c.bfdet);
        });
    }

    #[tokio::test]
    async fn filters_on_width_and_depth_with_one_query_returning_no_records() {
        let mut repo = MockMiscChannelRepo::new();

        repo.expect_shapes_with_depth()
            .with(predicate::eq(3.0))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        repo.expect_shapes_with_width()
            .with(predicate::eq(4.25))
            .returning(|_| Box::pin(async { Ok(vec![mc18x58(), mc13x50()]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-channels?detailing_depth=3.0&detailing_flange_width=4.25")
            .await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn returns_error_if_no_shapes_when_requesting_all() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels");
        response.await.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn bubbles_up_repo_err_getting_all() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_all()
            .returning(|| Box::pin(async { Err(MissingPropertyError::from("Ix"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels");
        response.await.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_aisc_manual_label_shape() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("MC18X58")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?aisc_manual_label=MC18X58");
        response.await.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_aisc_label_shape() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shape_with_aisc_manual_label()
            .with(predicate::eq(String::from("MC18X58")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("W"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?aisc_manual_label=MC18X58");
        response.await.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_no_edi_std_nomenclature_shape() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("MC18X58")))
            .returning(|_| Box::pin(async { Ok(None) }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?edi_std_nomenclature=MC18X58");
        response.await.assert_status_not_found();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_edi_std_nomenclature() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shape_with_edi_std_nomenclature()
            .with(predicate::eq(String::from("MC18X58")))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("W"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?edi_std_nomenclature=MC18X58");
        response.await.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_width() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shapes_with_width()
            .with(predicate::eq(4.125_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("bfdet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server
            .get("/misc-channels?detailing_flange_width=4.125")
            .await;
        response.assert_status_internal_server_error();
    }

    #[tokio::test]
    async fn handles_failure_filtering_on_depth() {
        let mut repo = MockMiscChannelRepo::new();
        repo.expect_shapes_with_depth()
            .with(predicate::eq(18.0_f64))
            .returning(|_| Box::pin(async { Err(MissingPropertyError::from("ddet"))? }));

        let app_state = AppStateDyn {
            repo: Arc::new(repo),
        };

        let app = Router::new()
            .route("/misc-channels", get(get_misc_channels))
            .with_state(Arc::new(app_state));

        let server = TestServer::new(app);
        let response = server.get("/misc-channels?detailing_depth=18.0");
        response.await.assert_status_internal_server_error();
    }
}
