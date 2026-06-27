use shapes::aisc_shapes::{MiscTee, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

/// Repository that manages data access for all misc. tee shapes
pub struct MiscTeeRepository {
    pool: Arc<PgPool>,
}

impl MiscTeeRepository {
    /// Creates a new instance of MiscTeeRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        MiscTeeRepository { pool }
    }
}

impl ShapeRepository<MiscTee> for MiscTeeRepository {
    fn all(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MiscTee>, Box<dyn Error>>> + Send + '_>> {
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
    y_lower,
    yp,
    bf_2tf,
    d_t,
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
    ro,
    h_upper,
    wgi
	FROM misc_tees;",
            )
            .fetch_all(&*self.pool)
            .await?;

            let wf_results = rows
                .into_iter()
                .map(|r| misc_tee_from_row(r))
                .collect::<Vec<_>>();
            if wf_results.iter().any(|r| r.is_err()) {
                for result in wf_results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(wf_results
                    .into_iter()
                    .map(|wf| wf.unwrap())
                    .collect::<Vec<_>>())
            }
        })
    }

    fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<MiscTee>, Box<dyn Error>>> + Send + '_>> {
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
    y_lower,
    yp,
    bf_2tf,
    d_t,
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
    ro,
    h_upper,
    wgi
	FROM misc_tees
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
            )
            .bind(edi_std_nomenclature)
            .fetch_optional(&*self.pool)
            .await?
            .map(misc_tee_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<MiscTee>, Box<dyn Error>>> + Send + '_>> {
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
    y_lower,
    yp,
    bf_2tf,
    d_t,
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
    ro,
    h_upper,
    wgi
	FROM misc_tees
	WHERE aisc_manual_label = $1
	LIMIT 1;",
            )
            .bind(aisc_manual_label)
            .fetch_optional(&*self.pool)
            .await?
            .map(misc_tee_from_row)
            .transpose()
        })
    }

    fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MiscTee>, Box<dyn Error>>> + Send + '_>> {
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
    y_lower,
    yp,
    bf_2tf,
    d_t,
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
    ro,
    h_upper,
    wgi
	FROM misc_tees
    WHERE ddet = $1;",
            )
            .bind(depth)
            .fetch_all(&*self.pool)
            .await?;

            let wf_results = rows
                .into_iter()
                .map(|r| misc_tee_from_row(r))
                .collect::<Vec<_>>();
            if wf_results.iter().any(|r| r.is_err()) {
                for result in wf_results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(wf_results
                    .into_iter()
                    .map(|wf| wf.unwrap())
                    .collect::<Vec<_>>())
            }
        })
    }

    fn shapes_with_width(
        &self,
        width: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<MiscTee>, Box<dyn Error>>> + Send + '_>> {
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
    y_lower,
    yp,
    bf_2tf,
    d_t,
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
    ro,
    h_upper,
    wgi
	FROM misc_tees
    WHERE bfdet = $1;",
            )
            .bind(width)
            .fetch_all(&*self.pool)
            .await?;

            let wf_results = rows
                .into_iter()
                .map(|r| misc_tee_from_row(r))
                .collect::<Vec<_>>();
            if wf_results.iter().any(|r| r.is_err()) {
                for result in wf_results.into_iter() {
                    if let Err(err) = result {
                        return Err(err);
                    }
                }
                unreachable!()
            } else {
                Ok(wf_results
                    .into_iter()
                    .map(|wf| wf.unwrap())
                    .collect::<Vec<_>>())
            }
        })
    }
}

// Helper Functions
fn misc_tee_from_row(row: PgRow) -> Result<MiscTee, Box<dyn Error>> {
    let maybe_wgi: Option<f64> = row.try_get("wgi")?;
    let builder = ShapeBuilder::new()
        .edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .aisc_manual_label(row.try_get("aisc_manual_label")?)
        .t_f(row.try_get("t_f")?)
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
        .y_lower(row.try_get("y_lower")?)
        .yp(row.try_get("yp")?)
        .bf_2tf(row.try_get("bf_2tf")?)
        .d_t(row.try_get("d_t")?)
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
        .ro(row.try_get("ro")?)
        .h_upper(row.try_get("h_upper")?);

    let builder = add_optional_wgi(builder, maybe_wgi);
    Ok(builder.try_build::<MiscTee>()?)
}

fn add_optional_wgi(builder: ShapeBuilder, maybe_wgi: Option<f64>) -> ShapeBuilder {
    match maybe_wgi {
        Some(wgi) => builder.wgi(wgi),
        None => builder,
    }
}
