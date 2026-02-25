use shape_repositories::pg_repositories::*;
use crate::app_state::AppState;

/// The service layer for the applicatoin 
pub struct ShapeService<AppState>{
    /// The app state for the application
    pub state: AppState
}

impl ShapeService<AppState<
        AngleRepository,
        CeeChannelRepository,
        DoubleAngleRepository,
        HPileRepository,
        HollowStructuralSectionRepository,
        RoundHollowStructuralSectionRepository,
        MiscBeamRepository,
        MiscChannelRepository,
        MiscTeeRepository,
        PipeRepository,
        StructuralBeamRepository,
        StructuralTeeRepository,
        WideFlangeRepository,
        WideFlangeTeeRepository,
    >>
{
    /// Creates a new instance of the service layer for the application
    /// using a Postgres implementation of AppState
    pub fn new(state: AppState<
        AngleRepository,
        CeeChannelRepository,
        DoubleAngleRepository,
        HPileRepository,
        HollowStructuralSectionRepository,
        RoundHollowStructuralSectionRepository,
        MiscBeamRepository,
        MiscChannelRepository,
        MiscTeeRepository,
        PipeRepository,
        StructuralBeamRepository,
        StructuralTeeRepository,
        WideFlangeRepository,
        WideFlangeTeeRepository,
    > ) -> Self {
        ShapeService {
            state
        }
    }
}