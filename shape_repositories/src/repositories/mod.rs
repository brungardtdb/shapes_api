//! This module is responsible for
//! managing data access for shapes

/// The repository responsible for
/// managing data access for wide flange shapes
pub mod wide_flange_repository;
/// The repository responsible for
/// managing data access for misc. beam shapes
pub mod misc_beam_repository;

pub use wide_flange_repository::WideFlangeRepository;
pub use misc_beam_repository::MiscBeamRepository;