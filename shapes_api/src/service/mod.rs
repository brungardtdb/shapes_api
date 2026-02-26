/// Contains the Postgres shape service implementation for the application
pub mod pg_shape_service;
/// Contains the shape service trait for the application
pub mod shape_service;

pub use pg_shape_service::PGShapeService;
pub use shape_service::ShapeService;
