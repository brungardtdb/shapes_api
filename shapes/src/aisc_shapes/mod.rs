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
/// Struct that models misc beams or "m" steel profiles
pub mod misc_beam;
/// Struct that models structural beams or "s" steel profiles
pub mod structural_beam;
/// Struct that models h-pile beams or "hp" steel profiles
pub mod h_pile;
/// Strut that models cee channels 
pub mod cee_channel;
/// Struct that models miscelaneous channels
pub mod misc_channel;
/// Struct that models hot-rolled angles
pub mod angle;
/// Struct that models wide-flange tee shapes
pub mod wide_flange_tee;
/// Struct that models misc. tee shapes
pub mod misc_tee;
/// Struct that models structural tee shapes
pub mod structural_tee;
/// Struct that models double angle shapes
pub mod double_angle;
/// Struct that models square and rectangular HSS shapes
pub mod hollow_structural_section;

pub use self::aisc_shape::AISCShape;
pub use self::errors::MissingPropertyError;
pub use self::shape_builder::ShapeBuilder;
pub use self::wide_flange::WideFlange;
pub use self::misc_beam::MiscBeam;
pub use self::structural_beam::StructuralBeam;
pub use self::h_pile::HPile;
pub use self::cee_channel::CeeChannel;
pub use self::misc_channel::MiscChannel;
pub use self::angle::Angle;
pub use self::wide_flange_tee::WideFlangeTee;
pub use self::misc_tee::MiscTee;
pub use self::structural_tee::StructuralTee;
pub use self::double_angle::DoubleAngle;
pub use self::hollow_structural_section::HollowStructuralSection;