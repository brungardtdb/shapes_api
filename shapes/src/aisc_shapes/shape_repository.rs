use std::error::Error;
/// Trait to manage retrieving shapes from a data source
pub trait ShapeRepository<T> {
    /// Retrieves all shapes
    fn all(&self) -> impl Future<Output = Result<Vec<T>, Box<dyn Error>>> + Send;
    /// Retrieves a shape based on it's EDI Std Nomenclature
    fn shape_with_edi_std_nomenclature(&self, edi_std_nomenclature: String) -> impl Future<Output = Result<T, Box<dyn Error>>> + Send;
    /// Retrieves a shape based on it's AISC Manual Label
    fn shape_with_aisc_manual_label(&self, aisc_manual_label: String) -> impl Future<Output = Result<T, Box<dyn Error>>> + Send;
    /// Retrieves a shape based on it's depth
    fn shapes_with_depth(&self, depth: f64) -> impl Future<Output = Result<Vec<T>, Box<dyn Error>>> + Send;
    /// Retrieves a shape based on it's width
    fn shapes_with_width(&self, width: f64) -> impl Future<Output = Result<Vec<T>, Box<dyn Error>>> + Send;
}