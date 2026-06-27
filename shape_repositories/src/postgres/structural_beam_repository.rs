use shapes::aisc_shapes::{ShapeBuilder, ShapeRepository, StructuralBeam};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

/// Repository that manages data access for all structural beam shapes
pub struct StructuralBeamRepository {
    pool: Arc<PgPool>,
}

impl StructuralBeamRepository {
    /// Creates a new instance of StructuralBeamRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        StructuralBeamRepository { pool }
    }
}

impl ShapeRepository<StructuralBeam> for StructuralBeamRepository {
    fn all(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<StructuralBeam>, Box<dyn Error>>> + Send + '_>>
    {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi
    FROM structural_beams;",
            )
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| structural_beam_from_row(r))
                .collect::<Vec<_>>();

            if results.iter().any(|r| r.is_err()) {
                for result in results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(results.into_iter().map(|b| b.unwrap()).collect::<Vec<_>>())
            }
        })
    }

    fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<StructuralBeam>, Box<dyn Error>>> + Send + '_>>
    {
        Box::pin(async move {
            sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi
    FROM structural_beams
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
            )
            .bind(edi_std_nomenclature)
            .fetch_optional(&*self.pool)
            .await?
            .map(structural_beam_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<StructuralBeam>, Box<dyn Error>>> + Send + '_>>
    {
        Box::pin(async move {
            sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi
    FROM structural_beams
	WHERE aisc_manual_label = $1
	LIMIT 1;",
            )
            .bind(aisc_manual_label)
            .fetch_optional(&*self.pool)
            .await?
            .map(structural_beam_from_row)
            .transpose()
        })
    }

    fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<StructuralBeam>, Box<dyn Error>>> + Send + '_>>
    {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi
    FROM structural_beams
    WHERE d_lower = $1;",
            )
            .bind(depth)
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| structural_beam_from_row(r))
                .collect::<Vec<_>>();
            if results.iter().any(|r| r.is_err()) {
                for result in results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(results.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>())
            }
        })
    }

    fn shapes_with_width(
        &self,
        width: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<StructuralBeam>, Box<dyn Error>>> + Send + '_>>
    {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi
    FROM structural_beams
    WHERE bf = $1;",
            )
            .bind(width)
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| structural_beam_from_row(r))
                .collect::<Vec<_>>();
            if results.iter().any(|r| r.is_err()) {
                for result in results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(results.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>())
            }
        })
    }
}

// Helper Functions
fn structural_beam_from_row(row: PgRow) -> Result<StructuralBeam, Box<dyn Error>> {
    let maybe_wgi: Option<f64> = row.try_get("wgi")?;

    let builder = ShapeBuilder::new()
        .edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .aisc_manual_label(row.try_get("aisc_manual_label")?)
        .w_upper(row.try_get("w_upper")?)
        .a_upper(row.try_get("a_upper")?)
        .d_lower(row.try_get("d_lower")?)
        .ddet(row.try_get("ddet")?)
        .bf(row.try_get("bf")?)
        .bfdet(row.try_get("bfdet")?)
        .tw(row.try_get("tw")?)
        .twdet(row.try_get("twdet")?)
        .twdet_2(row.try_get("twdet_2")?)
        .tf(row.try_get("tf")?)
        .tfdet(row.try_get("tfdet")?)
        .kdes(row.try_get("kdes")?)
        .kdet(row.try_get("kdet")?)
        .bf_2tf(row.try_get("bf_2tf")?)
        .h_tw(row.try_get("h_tw")?)
        .ix(row.try_get("ix")?)
        .zx(row.try_get("zx")?)
        .sx(row.try_get("sx")?)
        .rx(row.try_get("rx")?)
        .iy(row.try_get("iy")?)
        .zy(row.try_get("zy")?)
        .sy(row.try_get("sy")?)
        .ry(row.try_get("ry")?)
        .j_upper(row.try_get("j_upper")?)
        .cw(row.try_get("cw")?)
        .wno(row.try_get("wno")?)
        .sw1(row.try_get("sw1")?)
        .qf(row.try_get("qf")?)
        .qw(row.try_get("qw")?)
        .rts(row.try_get("rts")?)
        .ho(row.try_get("ho")?)
        .pa(row.try_get("pa")?)
        .pb(row.try_get("pb")?)
        .pc(row.try_get("pc")?)
        .pd(row.try_get("pd")?)
        .t(row.try_get("t")?);

    let builder = add_optional_wgi(builder, maybe_wgi);
    let maybe_sb = builder.try_build::<StructuralBeam>();
    match maybe_sb {
        Ok(sb) => Ok(sb),
        Err(err) => Err(Box::new(err)),
    }
}

fn add_optional_wgi(builder: ShapeBuilder, maybe_wgi: Option<f64>) -> ShapeBuilder {
    match maybe_wgi {
        Some(wgi) => builder.wgi(wgi),
        None => builder,
    }
}
