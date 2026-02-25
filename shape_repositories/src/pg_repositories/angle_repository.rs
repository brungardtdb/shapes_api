use shapes::aisc_shapes::{Angle, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::sync::Arc;

/// Repository that manages data access for all angle shapes
pub struct AngleRepository {
    pool: Arc<PgPool>,
}

impl AngleRepository {
    /// Creates a new instance of AngleRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        AngleRepository { pool }
    }
}

impl ShapeRepository<Angle> for AngleRepository {
    async fn all(&self) -> Result<Vec<Angle>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    kdes,
    kdet,
    x_lower,
    y_lower,
    xp,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    iz,
    rz,
    sz,
    j_upper,
    cw,
    ro,
    h_upper,
    tan_a,
    iw,
    za,
    zb,
    zc,
    wa,
    wb,
    wc,
    swa,
    swb,
    swc,
    sza,
    szb,
    szc,
    pa,
    pa_2,
    pb
    FROM angles;",
        )
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| angle_from_row(r))
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
    ) -> Result<Angle, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    kdes,
    kdet,
    x_lower,
    y_lower,
    xp,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    iz,
    rz,
    sz,
    j_upper,
    cw,
    ro,
    h_upper,
    tan_a,
    iw,
    za,
    zb,
    zc,
    wa,
    wb,
    wc,
    swa,
    swb,
    swc,
    sza,
    szb,
    szc,
    pa,
    pa_2,
    pb
    FROM angles 
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
        )
        .bind(edi_std_nomenclature)
        .fetch_one(&*self.pool)
        .await?;

        angle_from_row(row)
    }

    async fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Result<Angle, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    kdes,
    kdet,
    x_lower,
    y_lower,
    xp,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    iz,
    rz,
    sz,
    j_upper,
    cw,
    ro,
    h_upper,
    tan_a,
    iw,
    za,
    zb,
    zc,
    wa,
    wb,
    wc,
    swa,
    swb,
    swc,
    sza,
    szb,
    szc,
    pa,
    pa_2,
    pb
    FROM angles 
	WHERE aisc_manual_label = $1
	LIMIT 1;",
        )
        .bind(aisc_manual_label)
        .fetch_one(&*self.pool)
        .await?;

        angle_from_row(row)
    }

    async fn shapes_with_depth(&self, depth: f64) -> Result<Vec<Angle>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    kdes,
    kdet,
    x_lower,
    y_lower,
    xp,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    iz,
    rz,
    sz,
    j_upper,
    cw,
    ro,
    h_upper,
    tan_a,
    iw,
    za,
    zb,
    zc,
    wa,
    wb,
    wc,
    swa,
    swb,
    swc,
    sza,
    szb,
    szc,
    pa,
    pa_2,
    pb
    FROM angles 
    WHERE b_lower = $1;",
        )
        .bind(depth)
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| angle_from_row(r))
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

    async fn shapes_with_width(&self, width: f64) -> Result<Vec<Angle>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    kdes,
    kdet,
    x_lower,
    y_lower,
    xp,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    iz,
    rz,
    sz,
    j_upper,
    cw,
    ro,
    h_upper,
    tan_a,
    iw,
    za,
    zb,
    zc,
    wa,
    wb,
    wc,
    swa,
    swb,
    swc,
    sza,
    szb,
    szc,
    pa,
    pa_2,
    pb
    FROM angles 
    WHERE d_lower = $1;",
        )
        .bind(width)
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| angle_from_row(r))
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
fn angle_from_row(row: PgRow) -> Result<Angle, Box<dyn Error>> {
    let maybe_h_upper: Option<f64> = row.try_get("h_upper")?;
    let maybe_swb: Option<f64> = row.try_get("swb")?;

    let builder = ShapeBuilder::new()
        .with_edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .with_aisc_manual_label(row.try_get("aisc_manual_label")?)
        .with_w_upper(row.try_get("w_upper")?)
        .with_a_upper(row.try_get("a_upper")?)
        .with_d_lower(row.try_get("d_lower")?)
        .with_b_lower(row.try_get("b_lower")?)
        .with_t_lower(row.try_get("t_lower")?)
        .with_kdes(row.try_get("kdes")?)
        .with_kdet(row.try_get("kdet")?)
        .with_x_lower(row.try_get("x_lower")?)
        .with_y_lower(row.try_get("y_lower")?)
        .with_xp(row.try_get("xp")?)
        .with_yp(row.try_get("yp")?)
        .with_b_t(row.try_get("b_t")?)
        .with_ix(row.try_get("ix")?)
        .with_zx(row.try_get("zx")?)
        .with_sx(row.try_get("sx")?)
        .with_rx(row.try_get("rx")?)
        .with_iy(row.try_get("iy")?)
        .with_zy(row.try_get("zy")?)
        .with_sy(row.try_get("sy")?)
        .with_ry(row.try_get("ry")?)
        .with_iz(row.try_get("iz")?)
        .with_rz(row.try_get("rz")?)
        .with_sz(row.try_get("sz")?)
        .with_j_upper(row.try_get("j_upper")?)
        .with_cw(row.try_get("cw")?)
        .with_ro(row.try_get("ro")?)
        .with_tan_a(row.try_get("tan_a")?)
        .with_iw(row.try_get("iw")?)
        .with_za(row.try_get("za")?)
        .with_zb(row.try_get("zb")?)
        .with_zc(row.try_get("zc")?)
        .with_wa(row.try_get("wa")?)
        .with_wb(row.try_get("wb")?)
        .with_wc(row.try_get("wc")?)
        .with_swa(row.try_get("swa")?)
        .with_swc(row.try_get("swc")?)
        .with_sza(row.try_get("sza")?)
        .with_szb(row.try_get("szb")?)
        .with_szc(row.try_get("szc")?)
        .with_pa(row.try_get("pa")?)
        .with_pa_2(row.try_get("pa_2")?)
        .with_pb(row.try_get("pb")?);

    let builder = optional_h_upper(builder, maybe_h_upper);
    let buider = optional_swb(builder, maybe_swb);
    Ok(buider.try_build::<Angle>()?)
}

fn optional_h_upper(builder: ShapeBuilder, maybe_h_upper: Option<f64>) -> ShapeBuilder {
    match maybe_h_upper {
        Some(h_upper) => builder.with_h_upper(h_upper),
        None => builder,
    }
}

fn optional_swb(builder: ShapeBuilder, maybe_swb: Option<f64>) -> ShapeBuilder {
    match maybe_swb {
        Some(swb) => builder.with_swb(swb),
        None => builder,
    }
}
