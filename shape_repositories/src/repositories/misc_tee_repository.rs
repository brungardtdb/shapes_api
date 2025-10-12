use shapes::aisc_shapes::{MiscTee, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
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
    async fn all(&self) -> Result<Vec<MiscTee>, Box<dyn Error>> {
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
    }

    async fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Result<MiscTee, Box<dyn Error>> {
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
        .fetch_one(&*self.pool)
        .await?;

        misc_tee_from_row(row)
    }

    async fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Result<MiscTee, Box<dyn Error>> {
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
        .fetch_one(&*self.pool)
        .await?;

        misc_tee_from_row(row)
    }

    async fn shapes_with_depth(&self, depth: f64) -> Result<Vec<MiscTee>, Box<dyn Error>> {
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
    WHERE d_lower = $1;",
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
    }

    async fn shapes_with_width(&self, width: f64) -> Result<Vec<MiscTee>, Box<dyn Error>> {
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
    WHERE bf = $1;",
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
    }
}

// Helper Functions
fn misc_tee_from_row(row: PgRow) -> Result<MiscTee, Box<dyn Error>> {
    let maybe_wgi: Option<f64> = row.try_get("wgi")?;
    let builder = ShapeBuilder::new()
        .with_edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .with_aisc_manual_label(row.try_get("aisc_manual_label")?)
        .with_t_f(row.try_get("t_f")?)
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
        .with_y_lower(row.try_get("y_lower")?)
        .with_yp(row.try_get("yp")?)
        .with_bf_2tf(row.try_get("bf_2tf")?)
        .with_d_t(row.try_get("d_t")?)
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
        .with_ro(row.try_get("ro")?)
        .with_h_upper(row.try_get("h_upper")?);

    let builder = add_optional_wgi(builder, maybe_wgi);
    Ok(builder.try_build::<MiscTee>()?)
}

fn add_optional_wgi(builder: ShapeBuilder, maybe_wgi: Option<f64>) -> ShapeBuilder {
    match maybe_wgi {
        Some(wgi) => builder.with_wgi(wgi),
        None => builder,
    }
}
