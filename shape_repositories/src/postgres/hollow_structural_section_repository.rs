use shapes::aisc_shapes::{HollowStructuralSection, ShapeBuilder, ShapeRepository};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::pin::Pin;
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
    fn all(
        &self,
    ) -> Pin<
        Box<dyn Future<Output = Result<Vec<HollowStructuralSection>, Box<dyn Error>>> + Send + '_>,
    > {
        Box::pin(async move {
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
        })
    }

    fn shape_with_edi_std_nomenclature(
        &self,
        edi_std_nomenclature: String,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<HollowStructuralSection>, Box<dyn Error>>>
                + Send
                + '_,
        >,
    > {
        Box::pin(async move {
            sqlx::query(
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
            .fetch_optional(&*self.pool)
            .await?
            .map(hollow_structural_section_from_row)
            .transpose()
        })
    }

    fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<HollowStructuralSection>, Box<dyn Error>>>
                + Send
                + '_,
        >,
    > {
        Box::pin(async move {
            sqlx::query(
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
            .fetch_optional(&*self.pool)
            .await?
            .map(hollow_structural_section_from_row)
            .transpose()
        })
    }

    fn shapes_with_depth(
        &self,
        depth: f64,
    ) -> Pin<
        Box<dyn Future<Output = Result<Vec<HollowStructuralSection>, Box<dyn Error>>> + Send + '_>,
    > {
        Box::pin(async move {
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
        })
    }

    fn shapes_with_width(
        &self,
        width: f64,
    ) -> Pin<
        Box<dyn Future<Output = Result<Vec<HollowStructuralSection>, Box<dyn Error>>> + Send + '_>,
    > {
        Box::pin(async move {
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
        })
    }
}

// Helper Functions
fn hollow_structural_section_from_row(
    row: PgRow,
) -> Result<HollowStructuralSection, Box<dyn Error>> {
    Ok(ShapeBuilder::new()
        .edi_std_nomenclature(row.try_get("edi_std_nomenclature")?)
        .aisc_manual_label(row.try_get("aisc_manual_label")?)
        .w_upper(row.try_get("w_upper")?)
        .a_upper(row.try_get("a_upper")?)
        .ht(row.try_get("ht")?)
        .h(row.try_get("h")?)
        .b_upper(row.try_get("b_upper")?)
        .b_lower(row.try_get("b_lower")?)
        .t_nom(row.try_get("t_nom")?)
        .tdes(row.try_get("tdes")?)
        .b_tdes(row.try_get("b_tdes")?)
        .h_tdes(row.try_get("h_tdes")?)
        .ix(row.try_get("ix")?)
        .zx(row.try_get("zx")?)
        .sx(row.try_get("sx")?)
        .rx(row.try_get("rx")?)
        .iy(row.try_get("iy")?)
        .zy(row.try_get("zy")?)
        .sy(row.try_get("sy")?)
        .ry(row.try_get("ry")?)
        .j_upper(row.try_get("j_upper")?)
        .c_upper(row.try_get("c_upper")?)
        .try_build::<HollowStructuralSection>()?)
}
