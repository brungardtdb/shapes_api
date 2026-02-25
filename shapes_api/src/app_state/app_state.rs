use shape_repositories::pg_repositories::*;
use shapes::aisc_shapes::*;
use sqlx::PgPool;
use std::sync::Arc;

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
    pub angle_repo: Arc<AR>,
    pub channel_repo: Arc<CR>,
    pub double_angle_repo: Arc<DAR>,
    pub h_pile_repo: Arc<HR>,
    pub hss_repo: Arc<HSS>,
    pub hss_round_repo: Arc<HSSR>,
    pub misc_beam_repo: Arc<MR>,
    pub misc_channel_repo: Arc<MCH>,
    pub misc_tee_repo: Arc<MT>,
    pub pipe_repo: Arc<PIPE>,
    pub structural_beam_repo: Arc<SB>,
    pub structural_tee_repo: Arc<ST>,
    pub wide_flange_repo: Arc<WF>,
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
