use shape_repositories::repositories::PipeRepository;
use shapes::aisc_shapes::shape_repository::RoundShapeRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let repo = PipeRepository::new(Arc::clone(&conx));
    let all_shapes_result = repo.all().await;
    match all_shapes_result {
        Ok(shapes) => println!("There are {} pipe shapes", shapes.len()),
        Err(err) => println!("{}", err),
    }

    let edi_result = repo
        .shape_with_edi_std_nomenclature(String::from("Pipe2SCH40"))
        .await;
    match edi_result {
        Ok(shape) => println!("{}", shape.edi_std_nomenclature),
        Err(err) => println!("{}", err),
    }
    let lbl_result = repo
        .shape_with_aisc_manual_label(String::from("Pipe2STD"))
        .await;
    match lbl_result {
        Ok(shape) => println!("{}", shape.aisc_manual_label),
        Err(err) => println!("{}", err),
    }
    let diameter = 4.5;
    println!("Diameter: {}", &diameter);
    let depth_result = repo.shapes_with_diameter(diameter).await;
    match depth_result {
        Ok(shapes) => {
            for shape in shapes {
                println!("{}", shape.aisc_manual_label);
            }
        }
        Err(err) => println!("{}", err),
    }
    Ok(())
}
