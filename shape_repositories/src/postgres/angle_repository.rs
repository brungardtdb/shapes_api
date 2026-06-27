use shapes::aisc_shapes::{Angle, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
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
    fn all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Angle>, Box<dyn Error>>> + Send + '_>> {
        return Box::pin(async move {
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
            .fetch_all(&*self.pool);

            let results = rows
                .await?
                .into_iter()
                .map(|r| angle_from_row(r))
                .collect::<Vec<_>>();
            if results
                .iter()
                .any(|r: &Result<Angle, Box<dyn Error>>| r.is_err())
            {
                for result in results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(results
                    .into_iter()
                    .map(|r: Result<Angle, Box<dyn Error>>| r.unwrap())
                    .collect::<Vec<_>>())
            }
        });
    }

    fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Angle>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query(
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
            .fetch_optional(&*self.pool)
            .await?
            .map(angle_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Angle>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query(
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
            .fetch_optional(&*self.pool)
            .await?
            .map(angle_from_row)
            .transpose()
        })
    }

    fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Angle>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
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
        })
    }

    fn shapes_with_width(
        &self,
        width: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Angle>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
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
        })
    }
}

// Helper Functions
fn angle_from_row(row: PgRow) -> Result<Angle, Box<dyn Error>> {
    let maybe_h_upper: Option<f64> = row.try_get("h_upper")?;
    let maybe_swb: Option<f64> = row.try_get("swb")?;

    let builder = ShapeBuilder::new()
        .edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .aisc_manual_label(row.try_get("aisc_manual_label")?)
        .w_upper(row.try_get("w_upper")?)
        .a_upper(row.try_get("a_upper")?)
        .d_lower(row.try_get("d_lower")?)
        .b_lower(row.try_get("b_lower")?)
        .t_lower(row.try_get("t_lower")?)
        .kdes(row.try_get("kdes")?)
        .kdet(row.try_get("kdet")?)
        .x_lower(row.try_get("x_lower")?)
        .y_lower(row.try_get("y_lower")?)
        .xp(row.try_get("xp")?)
        .yp(row.try_get("yp")?)
        .b_t(row.try_get("b_t")?)
        .ix(row.try_get("ix")?)
        .zx(row.try_get("zx")?)
        .sx(row.try_get("sx")?)
        .rx(row.try_get("rx")?)
        .iy(row.try_get("iy")?)
        .zy(row.try_get("zy")?)
        .sy(row.try_get("sy")?)
        .ry(row.try_get("ry")?)
        .iz(row.try_get("iz")?)
        .rz(row.try_get("rz")?)
        .sz(row.try_get("sz")?)
        .j_upper(row.try_get("j_upper")?)
        .cw(row.try_get("cw")?)
        .ro(row.try_get("ro")?)
        .tan_a(row.try_get("tan_a")?)
        .iw(row.try_get("iw")?)
        .za(row.try_get("za")?)
        .zb(row.try_get("zb")?)
        .zc(row.try_get("zc")?)
        .wa(row.try_get("wa")?)
        .wb(row.try_get("wb")?)
        .wc(row.try_get("wc")?)
        .swa(row.try_get("swa")?)
        .swc(row.try_get("swc")?)
        .sza(row.try_get("sza")?)
        .szb(row.try_get("szb")?)
        .szc(row.try_get("szc")?)
        .pa(row.try_get("pa")?)
        .pa_2(row.try_get("pa_2")?)
        .pb(row.try_get("pb")?);

    let builder = optional_h_upper(builder, maybe_h_upper);
    let buider = optional_swb(builder, maybe_swb);
    Ok(buider.try_build::<Angle>()?)
}

fn optional_h_upper(builder: ShapeBuilder, maybe_h_upper: Option<f64>) -> ShapeBuilder {
    match maybe_h_upper {
        Some(h_upper) => builder.h_upper(h_upper),
        None => builder,
    }
}

fn optional_swb(builder: ShapeBuilder, maybe_swb: Option<f64>) -> ShapeBuilder {
    match maybe_swb {
        Some(swb) => builder.swb(swb),
        None => builder,
    }
}
