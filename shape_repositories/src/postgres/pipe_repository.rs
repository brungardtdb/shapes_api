use shapes::aisc_shapes::{Pipe, RoundShapeRepository, ShapeBuilder};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

/// Repository that manages data access for all pipe shapes
pub struct PipeRepository {
    pool: Arc<PgPool>,
}

impl PipeRepository {
    /// Creates a new instance of PipeRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        PipeRepository { pool }
    }
}

impl RoundShapeRepository<Pipe> for PipeRepository {
    fn all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Pipe>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    od,
    id,
    t_nom,
    tdes,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper
    FROM pipes;",
            )
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| pipe_from_row(r))
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
    ) -> Pin<Box<dyn Future<Output = Result<Option<Pipe>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    od,
    id,
    t_nom,
    tdes,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper
    FROM pipes 
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
            )
            .bind(edi_std_nomenclature)
            .fetch_optional(&*self.pool)
            .await?
            .map(pipe_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Pipe>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    od,
    id,
    t_nom,
    tdes,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper
    FROM pipes 
	WHERE aisc_manual_label = $1
	LIMIT 1;",
            )
            .bind(aisc_manual_label)
            .fetch_optional(&*self.pool)
            .await?
            .map(pipe_from_row)
            .transpose()
        })
    }

    fn shapes_with_diameter(
        &self,
        diameter: f64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Pipe>, Box<dyn Error>>> + Send + '_>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    od,
    id,
    t_nom,
    tdes,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper
    FROM pipes 
    WHERE od = $1;",
            )
            .bind(diameter)
            .fetch_all(&*self.pool)
            .await?;

            let results = rows
                .into_iter()
                .map(|r| pipe_from_row(r))
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
fn pipe_from_row(row: PgRow) -> Result<Pipe, Box<dyn Error>> {
    Ok(ShapeBuilder::new()
        .with_edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .with_aisc_manual_label(row.try_get("aisc_manual_label")?)
        .with_w_upper(row.try_get("w_upper")?)
        .with_a_upper(row.try_get("a_upper")?)
        .with_od(row.try_get("od")?)
        .with_id(row.try_get("id")?)
        .with_t_nom(row.try_get("t_nom")?)
        .with_tdes(row.try_get("tdes")?)
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
        .try_build::<Pipe>()?)
}
