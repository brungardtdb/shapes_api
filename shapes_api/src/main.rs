use shape_repositories::repositories::HollowStructuralSectionRepository;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let repo = HollowStructuralSectionRepository::new(Arc::clone(&conx));
    let all_shapes_result = repo.all().await;
    match all_shapes_result {
        Ok(shapes) => println!("There are {} structural tees", shapes.len()),
        Err(err) => println!("{}", err),
    }

    let edi_result = repo
        .shape_with_edi_std_nomenclature(String::from("HSS6X4X.500"))
        .await;
    match edi_result {
        Ok(shape) => println!("{}", shape.edi_std_nomenclature),
        Err(err) => println!("{}", err),
    }
    let lbl_result = repo
        .shape_with_aisc_manual_label(String::from("HSS6X4X1/2"))
        .await;
    match lbl_result {
        Ok(shape) => println!("{}", shape.aisc_manual_label),
        Err(err) => println!("{}", err),
    }
    let depth = 8.0;
    println!("Depth: {}", &depth);
    let depth_result = repo.shapes_with_depth(depth).await;
    match depth_result {
        Ok(shapes) => {
            for shape in shapes {
                println!("{}", shape.aisc_manual_label);
            }
        }
        Err(err) => println!("{}", err),
    }
    let width = 4.0;
    println!("Width: {}", &width);
    let shapes_result = repo.shapes_with_width(width).await;
    match shapes_result {
        Ok(shapes) => {
            for shape in shapes {
                println!("{}", shape.aisc_manual_label);
            }
        }
        Err(err) => println!("{}", err),
    }
    Ok(())
}
