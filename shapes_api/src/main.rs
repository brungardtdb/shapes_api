use axum::{Router, routing::get};
use shape_repositories::pg_repositories::WideFlangeRepository;
use shapes_api::handlers::aisc_handlers::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let wf_repo = Arc::new(WideFlangeRepository::new(conx));

    let app = Router::new()
        .route("/wide-flange/all", get(wide_flange_handler::get_all))
        .with_state(Arc::new(wide_flange_handler::AppStateDyn {
            repo: wf_repo.clone(),
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}
