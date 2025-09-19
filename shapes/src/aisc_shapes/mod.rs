//! This module is responsible for creating and modeling
//! AISC steel shapes

/// Manages errors for constructing steel profiles
pub mod errors;
/// Manages the construction of steel shapes
pub mod shape_builder;
/// Models wide flange steel profiles
pub mod wide_flange;
/// Models misc beams or "m" steel profiles
pub mod misc_beam;
/// Models structural beams or "s" steel profiles
pub mod structural_beam;
/// Models h-pile beams or "hp" steel profiles
pub mod h_pile;
/// Models cee channels 
pub mod cee_channel;
/// Models miscelaneous channels
pub mod misc_channel;
/// Models hot-rolled angles
pub mod angle;
/// Models wide-flange tee shapes
pub mod wide_flange_tee;
/// Models misc. tee shapes
pub mod misc_tee;
/// Models structural tee shapes
pub mod structural_tee;
/// Models double angle shapes
pub mod double_angle;
/// Models square and rectangular HSS shapes
pub mod hollow_structural_section;
/// Models round HSS shapes
pub mod round_hollow_structural_section;
///Struct that models pipe shapes
pub mod pipe;

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
pub use self::round_hollow_structural_section::RoundHollowStructuralSection;
pub use self::pipe::Pipe;