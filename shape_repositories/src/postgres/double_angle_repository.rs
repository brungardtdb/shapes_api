use shapes::aisc_shapes::{DoubleAngle, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
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
    fn all(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DoubleAngle>, Box<dyn Error>>> + Send + '_>> {
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
        })
    }

    fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<DoubleAngle>, Box<dyn Error>>> + Send + '_>>
    {
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
            .fetch_optional(&*self.pool)
            .await?
            .map(double_angle_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<DoubleAngle>, Box<dyn Error>>> + Send + '_>>
    {
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
            .fetch_optional(&*self.pool)
            .await?
            .map(double_angle_from_row)
            .transpose()
        })
    }

    fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DoubleAngle>, Box<dyn Error>>> + Send + '_>> {
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
        })
    }

    fn shapes_with_width(
        &self,
        width: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DoubleAngle>, Box<dyn Error>>> + Send + '_>> {
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
        })
    }
}

// Helper Functions
fn double_angle_from_row(row: PgRow) -> Result<DoubleAngle, Box<dyn Error>> {
    Ok(ShapeBuilder::new()
        .edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .aisc_manual_label(row.try_get("aisc_manual_label")?)
        .w_upper(row.try_get("w_upper")?)
        .a_upper(row.try_get("a_upper")?)
        .d_lower(row.try_get("d_lower")?)
        .b_lower(row.try_get("b_lower")?)
        .t_lower(row.try_get("t_lower")?)
        .y_lower(row.try_get("y_lower")?)
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
        .ro(row.try_get("ro")?)
        .h_upper(row.try_get("h_upper")?)
        .try_build::<DoubleAngle>()?)
}
