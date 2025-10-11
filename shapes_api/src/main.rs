use shape_repositories::repositories::MiscChannelRepository;
use shapes::aisc_shapes::shape_repository::ShapeRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let repo = MiscChannelRepository::new(Arc::clone(&conx));
    let all_shapes_result = repo.all().await;
    match all_shapes_result {
        Ok(shapes) => println!("There are {} misc. channels", shapes.len()),
        Err(err) => println!("{}", err),
    }

    let edi_result = repo
        .shape_with_edi_std_nomenclature(String::from("MC6X7"))
        .await;
    match edi_result {
        Ok(shape) => println!("{}", shape.edi_std_nomenclature),
        Err(err) => println!("{}", err),
    }
    let lbl_result = repo
        .shape_with_aisc_manual_label(String::from("MC6X7"))
        .await;
    match lbl_result {
        Ok(shape) => println!("{}", shape.aisc_manual_label),
        Err(err) => println!("{}", err),
    }
    let depth_result = repo.shapes_with_depth(6.0).await;
    match depth_result {
        Ok(shapes) => {
            for shape in shapes {
                println!("{}", shape.aisc_manual_label);
            }
        }
        Err(err) => println!("{}", err),
    }
    let shapes_result = repo.shapes_with_width(3.5).await;
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
