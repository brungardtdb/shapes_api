use shape_repositories::pg_repositories::WideFlangeRepository;
use shapes::aisc_shapes::ShapeRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let repo = WideFlangeRepository::new(conx);
    let beams = repo.all().await?;
    for b in beams {
        println!("{}", b.aisc_manual_label);
    }
    Ok(())
}
