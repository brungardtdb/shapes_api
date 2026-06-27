use shapes::aisc_shapes::{MiscBeam, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

/// Repository that manages data access for all misc. beam shapes
pub struct MiscBeamRepository {
    pool: Arc<PgPool>,
}

impl MiscBeamRepository {
    /// Creates a new instance of MiscBeamRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        MiscBeamRepository { pool }
    }
}

impl ShapeRepository<MiscBeam> for MiscBeamRepository {
    fn all(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MiscBeam>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
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
    k1,
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
    FROM misc_beams;",
            )
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| misc_beam_from_row(r))
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

    fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<MiscBeam>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
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
    k1,
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
    FROM misc_beams
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
            )
            .bind(edi_std_nomenclature)
            .fetch_optional(&*self.pool)
            .await?
            .map(misc_beam_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<MiscBeam>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
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
    k1,
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
    FROM misc_beams
	WHERE aisc_manual_label = $1
	LIMIT 1;",
            )
            .bind(aisc_manual_label)
            .fetch_optional(&*self.pool)
            .await?
            .map(misc_beam_from_row)
            .transpose()
        })
    }

    fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MiscBeam>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
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
    k1,
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
    FROM misc_beams
    WHERE d_lower = $1;",
            )
            .bind(depth)
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| misc_beam_from_row(r))
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
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MiscBeam>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
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
    k1,
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
    FROM misc_beams
    WHERE bf = $1;",
            )
            .bind(width)
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| misc_beam_from_row(r))
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
fn misc_beam_from_row(row: PgRow) -> Result<MiscBeam, Box<dyn Error>> {
    let maybe_t_f: Option<bool> = row.try_get("t_f")?;
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
        .k1(row.try_get("k1")?)
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

    let builder = add_optional_t_f(builder, maybe_t_f);
    let builder = add_optional_wgi(builder, maybe_wgi);
    let maybe_mb = builder.try_build::<MiscBeam>();
    match maybe_mb {
        Ok(mb) => Ok(mb),
        Err(err) => Err(Box::new(err)),
    }
}

fn add_optional_wgi(builder: ShapeBuilder, maybe_wgi: Option<f64>) -> ShapeBuilder {
    match maybe_wgi {
        Some(wgi) => builder.wgi(wgi),
        None => builder,
    }
}

fn add_optional_t_f(builder: ShapeBuilder, maybe_t_f: Option<bool>) -> ShapeBuilder {
    match maybe_t_f {
        Some(t_f) => builder.t_f(t_f),
        None => builder,
    }
}
