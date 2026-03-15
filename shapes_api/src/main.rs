use shapes_api::app_state::AppState;
use shapes_api::service::{PGShapeService, ShapeService};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let app_state = AppState::new(Arc::clone(&conx));
    let svc = PGShapeService::new(app_state);
    let pipes = svc.pipes().await?;
    for p in pipes {
        println!("{}", p.aisc_manual_label);
    }
    Ok(())
}
