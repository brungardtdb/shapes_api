use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};

#[derive(Debug)]
#[allow(dead_code)]
pub struct AISCShape<'std_nom, 'aisc_label> {
    edi_std_nomenclature: &'std_nom str,
    aisc_manual_label: &'aisc_label str,
    t_f: Option<bool>,
    w_upper: f64,
    a_upper: f64,
    d_lower: Option<f64>,
    ddet: Option<f64>,
    ht: Option<f64>,
    h: Option<f64>,
    od: Option<f64>,
    bf: Option<f64>,
    bfdet: Option<f64>,
    b_upper: Option<f64>,
    b_lower: Option<f64>,
    id: Option<f64>,
    tw: Option<f64>,
    twdet: Option<f64>,
    twdet_2: Option<f64>,
    tf: Option<f64>,
    tfdet: Option<f64>,
    t_lower: Option<f64>,
    t_nom: Option<f64>,
    tdes: Option<f64>,
    kdes: Option<f64>,
    kdet: Option<f64>,
    k1: Option<f64>,
    x_lower: Option<f64>,
    y_lower: Option<f64>,
    eo: Option<f64>,
    xp: Option<f64>,
    yp: Option<f64>,
    bf_2tf: Option<f64>,
    b_t: Option<f64>,
    b_tdes: Option<f64>,
    h_tw: Option<f64>,
    h_tdes: Option<f64>,
    d_t: Option<f64>,
    ix: f64,
    zx: f64,
    sx: f64,
    rx: f64,
    ly: f64,
    zy: f64,
    sy: f64,
    ry: f64,
    iz: Option<f64>,
    rz: Option<f64>,
    sz: Option<f64>,
    j_upper: Option<f64>,
    cw: Option<f64>,
    c_upper: Option<f64>,
    wno: Option<f64>,
    sw1: Option<f64>,
    sw2: Option<f64>,
    sw3: Option<f64>,
    qf: Option<f64>,
    qw: Option<f64>,
    ro: Option<f64>,
    h_upper: Option<f64>,
    tan_a: Option<f64>,
    iw: Option<f64>,
    za: Option<f64>,
    zb: Option<f64>,
    zc: Option<f64>,
    wa: Option<f64>,
    wb: Option<f64>,
    wc: Option<f64>,
    swa: Option<f64>,
    swb: Option<f64>,
    swc: Option<f64>,
    sza: Option<f64>,
    szb: Option<f64>,
    szc: Option<f64>,
    rts: Option<f64>,
    ho: Option<f64>,
    pa: Option<f64>,
    pa_2: Option<f64>,
    pb: Option<f64>,
    pc: Option<f64>,
    pd: Option<f64>,
    t: Option<f64>,
    wgi: Option<f64>,
    wgo: Option<f64>,
}

impl<'std_nom, 'aisc_label> TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>
    for AISCShape<'std_nom, 'aisc_label>
{
    type Error = MissingPropertyError;
    fn try_from(
        builder: ShapeBuilder<'std_nom, 'aisc_label>,
    ) -> Result<Self, MissingPropertyError> {
        Ok(AISCShape {
            edi_std_nomenclature: match &builder.edi_std_nomenclature {
                Some(nom) => *nom,
                None => {
                    return Err(MissingPropertyError::from("EDI Std Nomenclature"));
                }
            },
            aisc_manual_label: match &builder.aisc_manual_label {
                Some(label) => *label,
                None => {
                    return Err(MissingPropertyError::from("AISC Manual Label"));
                }
            },
            t_f: *&builder.t_f,
            w_upper: match &builder.w_upper {
                Some(w) => *w,
                None => {
                    return Err(MissingPropertyError::from("W"));
                }
            },
            a_upper: match &builder.a_upper {
                Some(a_upper) => *a_upper,
                None => {
                    return Err(MissingPropertyError::from("A"));
                }
            },
            d_lower: *&builder.d_lower,
            ddet: *&builder.ddet,
            ht: *&builder.ht,
            h: *&builder.h,
            od: *&builder.od,
            bf: *&builder.bf,
            bfdet: *&builder.bfdet,
            b_upper: *&builder.b_upper,
            b_lower: *&builder.b_lower,
            id: *&builder.id,
            tw: *&builder.tw,
            twdet: *&builder.twdet,
            twdet_2: *&builder.twdet_2,
            tf: *&builder.tf,
            tfdet: *&builder.tfdet,
            t_lower: *&builder.t_lower,
            t_nom: *&builder.t_nom,
            tdes: *&builder.tdes,
            kdes: *&builder.kdes,
            kdet: *&builder.kdet,
            k1: *&builder.k1,
            x_lower: *&builder.x_lower,
            y_lower: *&builder.y_lower,
            eo: *&builder.eo,
            xp: *&builder.xp,
            yp: *&builder.yp,
            bf_2tf: *&builder.bf_2tf,
            b_t: *&builder.b_t,
            b_tdes: *&builder.b_tdes,
            h_tw: *&builder.h_tw,
            h_tdes: *&builder.tdes,
            d_t: *&builder.d_t,
            ix: match &builder.ix {
                Some(ix) => *ix,
                None => {
                    return Err(MissingPropertyError::from("ix"));
                }
            },
            zx: match &builder.zx {
                Some(zx) => *zx,
                None => {
                    return Err(MissingPropertyError::from("zx"));
                }
            },
            sx: match &builder.sx {
                Some(sx) => *sx,
                None => {
                    return Err(MissingPropertyError::from("sx"));
                }
            },
            rx: match &builder.rx {
                Some(rx) => *rx,
                None => {
                    return Err(MissingPropertyError::from("rx"));
                }
            },
            ly: match &builder.ly {
                Some(ly) => *ly,
                None => {
                    return Err(MissingPropertyError::from("ly"));
                }
            },
            zy: match &builder.zy {
                Some(zy) => *zy,
                None => {
                    return Err(MissingPropertyError::from("zy"));
                }
            },
            sy: match &builder.sy {
                Some(sy) => *sy,
                None => {
                    return Err(MissingPropertyError::from("sy"));
                }
            },
            ry: match &builder.ry {
                Some(ry) => *ry,
                None => {
                    return Err(MissingPropertyError::from("ry"));
                }
            },
            iz: *&builder.iz,
            rz: *&builder.rz,
            sz: *&builder.sz,
            j_upper: *&builder.j_upper,
            cw: *&builder.cw,
            c_upper: *&builder.c_upper,
            wno: *&builder.wno,
            sw1: *&builder.sw1,
            sw2: *&builder.sw2,
            sw3: *&builder.sw3,
            qf: *&builder.qf,
            qw: *&builder.qw,
            ro: *&builder.ro,
            h_upper: *&builder.h_upper,
            tan_a: *&builder.tan_a,
            iw: *&builder.iw,
            za: *&builder.za,
            zb: *&builder.zb,
            zc: *&builder.zc,
            wa: *&builder.wa,
            wb: *&builder.wb,
            wc: *&builder.wc,
            swa: *&builder.swa,
            swb: *&builder.swb,
            swc: *&builder.swc,
            sza: *&builder.sza,
            szb: *&builder.szb,
            szc: *&builder.szc,
            rts: *&builder.rts,
            ho: *&builder.ho,
            pa: *&builder.pa,
            pa_2: *&builder.pa_2,
            pb: *&builder.pb,
            pc: *&builder.pc,
            pd: *&builder.pd,
            t: *&builder.t,
            wgi: *&builder.wgi,
            wgo: *&builder.wgo,
        })
    }
}