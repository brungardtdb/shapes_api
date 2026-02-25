use shapes::aisc_shapes::{DoubleAngle, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::sync::Arc;

/// Repository that manages data access for all double angle shapes
pub struct DoubleAngleRepository {
    pool: Arc<PgPool>,
}

impl DoubleAngleRepository {
    /// Creates a new instance of DoubleAngleRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        DoubleAngleRepository { pool }
    }
}

impl ShapeRepository<DoubleAngle> for DoubleAngleRepository {
    async fn all(&self) -> Result<Vec<DoubleAngle>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    y_lower,
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
    ro,
    h_upper
    FROM double_angles;",
        )
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| double_angle_from_row(r))
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
    ) -> Result<DoubleAngle, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    y_lower,
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
    ro,
    h_upper
    FROM double_angles 
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
        )
        .bind(edi_std_nomenclature)
        .fetch_one(&*self.pool)
        .await?;

        double_angle_from_row(row)
    }

    async fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Result<DoubleAngle, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    y_lower,
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
    ro,
    h_upper
    FROM double_angles 
	WHERE aisc_manual_label = $1
	LIMIT 1;",
        )
        .bind(aisc_manual_label)
        .fetch_one(&*self.pool)
        .await?;

        double_angle_from_row(row)
    }

    async fn shapes_with_depth(&self, depth: f64) -> Result<Vec<DoubleAngle>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    y_lower,
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
    ro,
    h_upper
    FROM double_angles 
    WHERE b_lower = $1;",
        )
        .bind(depth)
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| double_angle_from_row(r))
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

    async fn shapes_with_width(&self, width: f64) -> Result<Vec<DoubleAngle>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    y_lower,
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
    ro,
    h_upper
    FROM double_angles 
    WHERE d_lower = $1;",
        )
        .bind(width)
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| double_angle_from_row(r))
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
fn double_angle_from_row(row: PgRow) -> Result<DoubleAngle, Box<dyn Error>> {
    Ok(ShapeBuilder::new()
        .with_edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .with_aisc_manual_label(row.try_get("aisc_manual_label")?)
        .with_w_upper(row.try_get("w_upper")?)
        .with_a_upper(row.try_get("a_upper")?)
        .with_d_lower(row.try_get("d_lower")?)
        .with_b_lower(row.try_get("b_lower")?)
        .with_t_lower(row.try_get("t_lower")?)
        .with_y_lower(row.try_get("y_lower")?)
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
        .with_ro(row.try_get("ro")?)
        .with_h_upper(row.try_get("h_upper")?)
        .try_build::<DoubleAngle>()?)
}
