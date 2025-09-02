//! This module is responsible for creating and modeling
//! AISC steel shapes

/// Generic AISC Shape, will be removed in the future
pub mod aisc_shape;
/// Manages errors for constructing steel profiles
pub mod errors;
/// Manages the construction of steel shapes
pub mod shape_builder;
/// Struct that models wide flange steel profiles
pub mod wide_flange;
/// Struc that models misc beams or "m" steel profiles
pub mod misc_beam;

pub use self::aisc_shape::AISCShape;
pub use self::errors::MissingPropertyError;
pub use self::shape_builder::ShapeBuilder;
pub use self::wide_flange::WideFlange;
pub use self::misc_beam::MiscBeam;
