use shapes::aisc_shapes::{HollowStructuralSection, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::sync::Arc;

/// Repository that manages data access for all HSS shapes
pub struct HollowStructuralSectionRepository {
    pool: Arc<PgPool>,
}

impl HollowStructuralSectionRepository {
    /// Creates a new instance of HollowStructuralSectionRepository type
    /// Takes a pool containing the Postgres database connection
    pub fn new(pool: Arc<PgPool>) -> Self {
        HollowStructuralSectionRepository { pool }
    }
}

impl ShapeRepository<HollowStructuralSection> for HollowStructuralSectionRepository {
    async fn all(&self) -> Result<Vec<HollowStructuralSection>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    ht,
    h,
    b_upper,
    b_lower,
    t_nom,
    tdes,
    b_tdes,
    h_tdes,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    c_upper
    FROM hollow_structural_sections;",
        )
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| hollow_structural_section_from_row(r))
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
    ) -> Result<HollowStructuralSection, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    ht,
    h,
    b_upper,
    b_lower,
    t_nom,
    tdes,
    b_tdes,
    h_tdes,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    c_upper
    FROM hollow_structural_sections 
	WHERE edi_std_nomenclature = $1
	LIMIT 1;",
        )
        .bind(edi_std_nomenclature)
        .fetch_one(&*self.pool)
        .await?;

        hollow_structural_section_from_row(row)
    }

    async fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Result<HollowStructuralSection, Box<dyn Error>> {
        let row = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    ht,
    h,
    b_upper,
    b_lower,
    t_nom,
    tdes,
    b_tdes,
    h_tdes,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    c_upper
    FROM hollow_structural_sections 
	WHERE aisc_manual_label = $1
	LIMIT 1;",
        )
        .bind(aisc_manual_label)
        .fetch_one(&*self.pool)
        .await?;

        hollow_structural_section_from_row(row)
    }

    async fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Result<Vec<HollowStructuralSection>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    ht,
    h,
    b_upper,
    b_lower,
    t_nom,
    tdes,
    b_tdes,
    h_tdes,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    c_upper
    FROM hollow_structural_sections 
    WHERE ht = $1;",
        )
        .bind(depth)
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| hollow_structural_section_from_row(r))
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

    async fn shapes_with_width(
        &self,
        width: f64,
    ) -> Result<Vec<HollowStructuralSection>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT 
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    ht,
    h,
    b_upper,
    b_lower,
    t_nom,
    tdes,
    b_tdes,
    h_tdes,
    ix,
    zx,
    sx,
    rx,
    iy,
    zy,
    sy,
    ry,
    j_upper,
    c_upper
    FROM hollow_structural_sections 
    WHERE b_upper = $1;",
        )
        .bind(width)
        .fetch_all(&*self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|r| hollow_structural_section_from_row(r))
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
fn hollow_structural_section_from_row(
    row: PgRow,
) -> Result<HollowStructuralSection, Box<dyn Error>> {
    Ok(ShapeBuilder::new()
        .with_edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .with_aisc_manual_label(row.try_get("aisc_manual_label")?)
        .with_w_upper(row.try_get("w_upper")?)
        .with_a_upper(row.try_get("a_upper")?)
        .with_ht(row.try_get("ht")?)
        .with_h(row.try_get("h")?)
        .with_b_upper(row.try_get("b_upper")?)
        .with_b_lower(row.try_get("b_lower")?)
        .with_t_nom(row.try_get("t_nom")?)
        .with_tdes(row.try_get("tdes")?)
        .with_b_tdes(row.try_get("b_tdes")?)
        .with_h_tdes(row.try_get("h_tdes")?)
        .with_ix(row.try_get("ix")?)
        .with_zx(row.try_get("zx")?)
        .with_sx(row.try_get("sx")?)
        .with_rx(row.try_get("rx")?)
        .with_iy(row.try_get("iy")?)
        .with_zy(row.try_get("zy")?)
        .with_sy(row.try_get("sy")?)
        .with_ry(row.try_get("ry")?)
        .with_j_upper(row.try_get("j_upper")?)
        .with_c_upper(row.try_get("c_upper")?)
        .try_build::<HollowStructuralSection>()?)
}
