use axum::{Router, routing::get};
use shape_repositories::postgres::*;
use shapes_api::handlers::aisc_handlers::*;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);

    let app = Router::new()
        .route("/aisc/angle", get(angle_handler::get_angles))
        .with_state(Arc::new(angle_handler::AppStateDyn {
            repo: Arc::new(AngleRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/cee-channel",
            get(cee_channel_handler::get_cee_channels),
        )
        .with_state(Arc::new(cee_channel_handler::AppStateDyn {
            repo: Arc::new(CeeChannelRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/double-angle",
            get(double_angle_handler::get_double_angles),
        )
        .with_state(Arc::new(double_angle_handler::AppStateDyn {
            repo: Arc::new(DoubleAngleRepository::new(conx.clone())),
        }))
        .route("/aisc/h-pile", get(h_pile_handler::get_h_piles))
        .with_state(Arc::new(h_pile_handler::AppStateDyn {
            repo: Arc::new(HPileRepository::new(conx.clone())),
        }))
        .route("/aisc/misc-beam", get(misc_beam_handler::get_misc_beams))
        .with_state(Arc::new(misc_beam_handler::AppStateDyn {
            repo: Arc::new(MiscBeamRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/misc-channel",
            get(misc_channel_handler::get_misc_channels),
        )
        .with_state(Arc::new(misc_channel_handler::AppStateDyn {
            repo: Arc::new(MiscChannelRepository::new(conx.clone())),
        }))
        .route("/aisc/misc-tee", get(misc_tee_handler::get_misc_tees))
        .with_state(Arc::new(misc_tee_handler::AppStateDyn {
            repo: Arc::new(MiscTeeRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/wide-flange",
            get(wide_flange_handler::get_wide_flanges),
        )
        .with_state(Arc::new(wide_flange_handler::AppStateDyn {
            repo: Arc::new(WideFlangeRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/hollow-structural-section",
            get(hollow_structural_section_handler::get_hollow_structural_sections),
        )
        .with_state(Arc::new(hollow_structural_section_handler::AppStateDyn {
            repo: Arc::new(HollowStructuralSectionRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/round-hollow-structural-section",
            get(round_hollow_structural_section_handler::get_round_hollow_structural_sections),
        )
        .with_state(Arc::new(
            round_hollow_structural_section_handler::AppStateDyn {
                repo: Arc::new(RoundHollowStructuralSectionRepository::new(conx.clone())),
            },
        ))
        .route(
            "/aisc/structural-beam",
            get(structural_beam_handler::get_structural_beams),
        )
        .with_state(Arc::new(structural_beam_handler::AppStateDyn {
            repo: Arc::new(StructuralBeamRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/structural-tee",
            get(structural_tee_handler::get_structural_tees),
        )
        .with_state(Arc::new(structural_tee_handler::AppStateDyn {
            repo: Arc::new(StructuralTeeRepository::new(conx.clone())),
        }))
        .route(
            "/aisc/wide-flange-tee",
            get(wide_flange_tee_handler::get_wide_flange_tees),
        )
        .with_state(Arc::new(wide_flange_tee_handler::AppStateDyn {
            repo: Arc::new(WideFlangeTeeRepository::new(conx.clone())),
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
