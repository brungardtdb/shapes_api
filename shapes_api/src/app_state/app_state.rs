use shape_repositories::pg_repositories::*;
use shapes::aisc_shapes::*;
use sqlx::PgPool;
use std::sync::Arc;

/// Manages the application state
/// Holds all AISC shape repositories and all things stateful
pub struct AppState<
    AR: ShapeRepository<Angle>,
    CR: ShapeRepository<CeeChannel>,
    DAR: ShapeRepository<DoubleAngle>,
    HR: ShapeRepository<HPile>,
    HSS: ShapeRepository<HollowStructuralSection>,
    HSSR: RoundShapeRepository<RoundHollowStructuralSection>,
    MR: ShapeRepository<MiscBeam>,
    MCH: ShapeRepository<MiscChannel>,
    MT: ShapeRepository<MiscTee>,
    PIPE: RoundShapeRepository<Pipe>,
    SB: ShapeRepository<StructuralBeam>,
    ST: ShapeRepository<StructuralTee>,
    WF: ShapeRepository<WideFlange>,
    WT: ShapeRepository<WideFlangeTee>,
> {
    /// The AISC angle repository
    pub angle_repo: Arc<AR>,
    /// The AISC chanel repository
    pub channel_repo: Arc<CR>,
    /// The AISC double angle repository
    pub double_angle_repo: Arc<DAR>,
    /// The AISC H-Pile repository
    pub h_pile_repo: Arc<HR>,
    /// The AISC HSS repository
    pub hss_repo: Arc<HSS>,
    /// The AISC HSS round repository
    pub hss_round_repo: Arc<HSSR>,
    /// The AISC misc. beam repository
    pub misc_beam_repo: Arc<MR>,
    /// The AISC misc. channel repository
    pub misc_channel_repo: Arc<MCH>,
    /// The AISC misc. tee repository
    pub misc_tee_repo: Arc<MT>,
    /// The AISC pipe repository
    pub pipe_repo: Arc<PIPE>,
    /// The AISC structural beam repository
    pub structural_beam_repo: Arc<SB>,
    /// The AISC structural tee repository
    pub structural_tee_repo: Arc<ST>,
    /// The AISC wide-flange repository
    pub wide_flange_repo: Arc<WF>,
    /// The AISC wide-flange tee repository
    pub wide_flange_tee_repo: Arc<WT>,
}

impl
    AppState<
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
    >
{
    /// Creates a new instance if AppState given Postgres AISC shape repositories
    pub fn new(pool: Arc<PgPool>) -> Self {
        AppState {
            angle_repo: Arc::new(AngleRepository::new(Arc::clone(&pool))),
            channel_repo: Arc::new(CeeChannelRepository::new(Arc::clone(&pool))),
            double_angle_repo: Arc::new(DoubleAngleRepository::new(Arc::clone(&pool))),
            h_pile_repo: Arc::new(HPileRepository::new(Arc::clone(&pool))),
            hss_repo: Arc::new(HollowStructuralSectionRepository::new(Arc::clone(&pool))),
            hss_round_repo: Arc::new(RoundHollowStructuralSectionRepository::new(Arc::clone(
                &pool,
            ))),
            misc_beam_repo: Arc::new(MiscBeamRepository::new(Arc::clone(&pool))),
            misc_channel_repo: Arc::new(MiscChannelRepository::new(Arc::clone(&pool))),
            misc_tee_repo: Arc::new(MiscTeeRepository::new(Arc::clone(&pool))),
            pipe_repo: Arc::new(PipeRepository::new(Arc::clone(&pool))),
            structural_beam_repo: Arc::new(StructuralBeamRepository::new(Arc::clone(&pool))),
            structural_tee_repo: Arc::new(StructuralTeeRepository::new(Arc::clone(&pool))),
            wide_flange_repo: Arc::new(WideFlangeRepository::new(Arc::clone(&pool))),
            wide_flange_tee_repo: Arc::new(WideFlangeTeeRepository::new(Arc::clone(&pool))),
        }
    }
}
