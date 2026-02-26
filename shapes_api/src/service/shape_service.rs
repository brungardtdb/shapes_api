use crate::dto::aisc_shapes::*;
use std::error::Error;

/// Trait for the service that will manage all steel shapes
pub trait ShapeService: Send + Sync + 'static {
    /// Gets all angle profiles
    fn angles(&self) -> impl Future<Output = Result<Vec<Angle>, Box<dyn Error>>> + Send;
}
