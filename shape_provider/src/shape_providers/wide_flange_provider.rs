use shapes::aisc_shapes::{ShapeBuilder, ShapeRepository, WideFlange};
use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use std::error::Error;
use std::sync::Arc;

pub struct WideFlangeProvider {
    pool: Arc<PgPool>,
}

impl WideFlangeProvider {
    pub fn new(pool: Arc<PgPool>) -> Self {
        WideFlangeProvider { pool }
    }
}

impl ShapeRepository<WideFlange> for WideFlangeProvider {
    async fn all(&self) -> Result<Vec<WideFlange>, Box<dyn Error>> {
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
    wgi,
    wgo
	FROM wide_flanges;",
        )
        .fetch_all(&*self.pool)
        .await?;

        let wf_results = rows
            .into_iter()
            .map(|r| wide_flange_from_row(r))
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
    ) -> Result<WideFlange, Box<dyn Error>> {
        todo!("Get wide flange shape with matching edi std nomenclature");
    }

    async fn shape_with_aisc_manual_label(
        &self,
        aisc_manual_label: String,
    ) -> Result<WideFlange, Box<dyn Error>> {
        todo!("Get wide flange shape with matching aisc manual label");
    }

    async fn shapes_with_depth(&self, depth: f64) -> Result<Vec<WideFlange>, Box<dyn Error>> {
        todo!("Get all wide flange shapes with matching depth");
    }

    async fn shapes_with_width(&self, width: f64) -> Result<Vec<WideFlange>, Box<dyn Error>> {
        todo!("Get all wide flange shapes with matching width")
    }
}

// Helper Functions
fn wide_flange_from_row(row: PgRow) -> Result<WideFlange, Box<dyn Error>> {
    let maybe_wgo: Option<f64> = row.try_get("wgo")?;
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
        .with_t(row.try_get("t")?)
        .with_wgi(row.try_get("wgi")?);

    let builder = add_optional_wgo(builder, maybe_wgo);
    let maybe_wf = builder.try_build::<WideFlange>();
    match maybe_wf {
        Ok(wf) => Ok(wf),
        Err(err) => Err(Box::new(err)),
    }
}

fn add_optional_wgo(builder: ShapeBuilder, maybe_wgo: Option<f64>) -> ShapeBuilder {
    match maybe_wgo {
        Some(wgo) => {
            builder.with_wgo(wgo)
        },
        None => {
            builder
        }
    }
}