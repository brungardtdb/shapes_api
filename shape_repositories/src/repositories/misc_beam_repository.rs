use shapes::aisc_shapes::{MiscBeam, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
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
    async fn all(&self) -> Result<Vec<MiscBeam>, Box<dyn Error>> {
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
    }

    async fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Result<MiscBeam, Box<dyn Error>> {
        let row = sqlx::query(
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
        .fetch_one(&*self.pool)
        .await?;

        misc_beam_from_row(row)
    }

    async fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Result<MiscBeam, Box<dyn Error>> {
        let row = sqlx::query(
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
        .fetch_one(&*self.pool)
        .await?;

        misc_beam_from_row(row)
    }

    async fn shapes_with_depth(&self, depth: f64) -> Result<Vec<MiscBeam>, Box<dyn Error>> {
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
    }

    async fn shapes_with_width(&self, width: f64) -> Result<Vec<MiscBeam>, Box<dyn Error>> {
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
    }
}

// Helper Functions
fn misc_beam_from_row(row: PgRow) -> Result<MiscBeam, Box<dyn Error>> {
    let maybe_t_f: Option<bool> = row.try_get("t_f")?;
    let maybe_wgi: Option<f64> = row.try_get("wgi")?;

    let builder = ShapeBuilder::new()
        .with_edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .with_aisc_manual_label(row.try_get("aisc_manual_label")?)
        .with_w_upper(row.try_get("w_upper")?)
        .with_a_upper(row.try_get("a_upper")?)
        .with_d_lower(row.try_get("d_lower")?)
        .with_ddet(row.try_get("ddet")?)
        .with_bf(row.try_get("bf")?)
        .with_bfdet(row.try_get("bfdet")?)
        .with_tw(row.try_get("tw")?)
        .with_twdet(row.try_get("twdet")?)
        .with_twdet_2(row.try_get("twdet_2")?)
        .with_tf(row.try_get("tf")?)
        .with_tfdet(row.try_get("tfdet")?)
        .with_kdes(row.try_get("kdes")?)
        .with_kdet(row.try_get("kdet")?)
        .with_k1(row.try_get("k1")?)
        .with_bf_2tf(row.try_get("bf_2tf")?)
        .with_h_tw(row.try_get("h_tw")?)
        .with_ix(row.try_get("ix")?)
        .with_zx(row.try_get("zx")?)
        .with_sx(row.try_get("sx")?)
        .with_rx(row.try_get("rx")?)
        .with_iy(row.try_get("iy")?)
        .with_zy(row.try_get("zy")?)
        .with_sy(row.try_get("sy")?)
        .with_ry(row.try_get("ry")?)
        .with_j_upper(row.try_get("j_upper")?)
        .with_cw(row.try_get("cw")?)
        .with_wno(row.try_get("wno")?)
        .with_sw1(row.try_get("sw1")?)
        .with_qf(row.try_get("qf")?)
        .with_qw(row.try_get("qw")?)
        .with_rts(row.try_get("rts")?)
        .with_ho(row.try_get("ho")?)
        .with_pa(row.try_get("pa")?)
        .with_pb(row.try_get("pb")?)
        .with_pc(row.try_get("pc")?)
        .with_pd(row.try_get("pd")?)
        .with_t(row.try_get("t")?);

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
        Some(wgi) => builder.with_wgi(wgi),
        None => builder,
    }
}

fn add_optional_t_f(builder: ShapeBuilder, maybe_t_f: Option<bool>) -> ShapeBuilder {
    match maybe_t_f {
        Some(t_f) => builder.with_t_f(t_f),
        None => builder,
    }
}
