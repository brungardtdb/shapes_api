use shape_provider::shape_providers::WideFlangeProvider;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::{Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let provider = WideFlangeProvider::new(Arc::clone(&conx));
    let shapes = &provider.all().await?;
    println!("There are {} wide flange shapes", shapes.len());
    Ok(())
}
