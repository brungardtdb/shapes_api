use shapes_api::app_state::AppState;
use shapes_api::service::PGShapeService;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let conx = Arc::new(pool);
    let app_state = AppState::new(Arc::clone(&conx));
    let _svc = PGShapeService::new(app_state);
    // let all_shapes_result = &app_state.pipe_repo.all().await;
    // match all_shapes_result {
    //     Ok(shapes) => println!("There are {} pipe shapes", shapes.len()),
    //     Err(err) => println!("{}", err),
    // }

    // let edi_result = &app_state
    //     .pipe_repo
    //     .shape_with_edi_std_nomenclature(String::from("Pipe2SCH40"))
    //     .await;
    // match edi_result {
    //     Ok(shape) => println!("{}", shape.edi_std_nomenclature),
    //     Err(err) => println!("{}", err),
    // }
    // let lbl_result = &app_state
    //     .pipe_repo
    //     .shape_with_aisc_manual_label(String::from("Pipe2STD"))
    //     .await;
    // match lbl_result {
    //     Ok(shape) => println!("{}", shape.aisc_manual_label),
    //     Err(err) => println!("{}", err),
    // }
    // let diameter = 4.5;
    // println!("Diameter: {}", &diameter);
    // let depth_result = &app_state.pipe_repo.shapes_with_diameter(diameter).await;
    // match depth_result {
    //     Ok(shapes) => {
    //         for shape in shapes {
    //             println!("{}", shape.aisc_manual_label);
    //         }
    //     }
    //     Err(err) => println!("{}", err),
    // }
    Ok(())
}
