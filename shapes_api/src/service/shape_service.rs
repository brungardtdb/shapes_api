use crate::dto::aisc_shapes::*;
use std::error::Error;

/// Trait for the service that will manage all steel shapes
pub trait ShapeService: Send + Sync + 'static {
    /// Gets all angle profiles
    fn angles(&self) -> impl Future<Output = Result<Vec<Angle>, Box<dyn Error>>> + Send;
    /// Gets all cee channel profiles
    fn channels(&self) -> impl Future<Output = Result<Vec<CeeChannel>, Box<dyn Error>>> + Send;
    /// Gets all double angle profiles
    fn double_angles(
        &self,
    ) -> impl Future<Output = Result<Vec<DoubleAngle>, Box<dyn Error>>> + Send;
    /// Gets all h-pile profiles
    fn h_piles(&self) -> impl Future<Output = Result<Vec<HPile>, Box<dyn Error>>> + Send;
    /// Gets all HSS profiles
    fn hss(
        &self,
    ) -> impl Future<Output = Result<Vec<HollowStructuralSection>, Box<dyn Error>>> + Send;
    /// Gets all misc. beams
    fn misc_beams(&self) -> impl Future<Output = Result<Vec<MiscBeam>, Box<dyn Error>>> + Send;
    /// Gets all misc. channels
    fn misc_channels(
        &self,
    ) -> impl Future<Output = Result<Vec<MiscChannel>, Box<dyn Error>>> + Send;
    /// Gets all misc tees
    fn misc_tees(&self) -> impl Future<Output = Result<Vec<MiscTee>, Box<dyn Error>>> + Send;
    /// Gets all pipes
    fn pipes(&self) -> impl Future<Output = Result<Vec<Pipe>, Box<dyn Error>>> + Send;
    /// Gets all round HSS members
    fn hss_round(
        &self,
    ) -> impl Future<Output = Result<Vec<RoundHollowStructuralSection>, Box<dyn Error>>> + Send;
    /// Gets all structural beams
    fn structural_beams(
        &self,
    ) -> impl Future<Output = Result<Vec<StructuralBeam>, Box<dyn Error>>> + Send;
    /// Gets all structural tees
    fn structural_tees(
        &self,
    ) -> impl Future<Output = Result<Vec<StructuralTee>, Box<dyn Error>>> + Send;
    /// Gets all wide flange tees
    fn wide_flange_tees(
        &self,
    ) -> impl Future<Output = Result<Vec<WideFlangeTee>, Box<dyn Error>>> + Send;
    /// Gets all wide flange beams
    fn wide_flange_beams(
        &self,
    ) -> impl Future<Output = Result<Vec<WideFlange>, Box<dyn Error>>> + Send;
}
