use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};

#[derive(Debug)]
#[allow(dead_code)]
pub struct WideFlange<'std_nom, 'aisc_label> {
    edi_std_nomenclature: &'std_nom str,
    aisc_manual_label: &'aisc_label str,
    t_f: bool,
    w_upper: f64,
    a_upper: f64,
    d_lower: f64,
    ddet: f64,
    bf: f64,
    bfdet: f64,
    tw: f64,
    twdet: f64,
    twdet_2: f64,
    tf: f64,
    tfdet: f64,
    kdes: f64,
    kdet: f64,
    k1: f64,
    bf_2tf: f64,
    h_tw: f64,
    ix: f64,
    zx: f64,
    sx: f64,
    rx: f64,
    iy: f64,
    zy: f64,
    sy: f64,
    ry: f64,
    j_upper: f64,
    cw: f64,
    wno: f64,
    sw1: f64,
    qf: f64,
    qw: f64,
    rts: f64,
    ho: f64,
    pa: f64,
    pb: f64,
    pc: f64,
    pd: f64,
    t: f64,
    wgi: f64,
    wgo: Option<f64>,
}

impl<'std_nom, 'aisc_label> TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>
    for WideFlange<'std_nom, 'aisc_label>
{
    type Error = MissingPropertyError;
    fn try_from(
        builder: ShapeBuilder<'std_nom, 'aisc_label>,
    ) -> Result<Self, MissingPropertyError> {
        Ok(WideFlange {
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
            t_f: match *&builder.t_f {
                Some(t_f) => t_f,
                None => {
                    return Err(MissingPropertyError::from("T_F"));
                }
            },
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
            d_lower: match *&builder.d_lower {
                Some(d_lower) => d_lower,
                None => {
                    return Err(MissingPropertyError::from("d"));
                }
            },
            ddet: match *&builder.ddet {
                Some(ddet) => ddet,
                None => {
                    return Err(MissingPropertyError::from("ddet"));
                }
            },
            bf: match *&builder.bf {
                Some(bf) => bf,
                None => {
                    return Err(MissingPropertyError::from("bf"));
                }
            },
            bfdet: match *&builder.bfdet {
                Some(bfdet) => bfdet,
                None => {
                    return Err(MissingPropertyError::from("bfdet"));
                }
            },
            tw: match *&builder.tw {
                Some(tw) => tw,
                None => {
                    return Err(MissingPropertyError::from("tw"));
                }
            },
            twdet: match *&builder.twdet {
                Some(twdet) => twdet,
                None => {
                    return Err(MissingPropertyError::from("twdet"));
                }
            },
            twdet_2: match *&builder.twdet_2 {
                Some(twdet) => twdet,
                None => {
                    return Err(MissingPropertyError::from("twdet/2"));
                }
            },
            tf: match *&builder.tf {
                Some(tf) => tf,
                None => {
                    return Err(MissingPropertyError::from("tf"));
                }
            },
            tfdet: match *&builder.tfdet {
                Some(tfdet) => tfdet,
                None => {
                    return Err(MissingPropertyError::from("tfdet"));
                }
            },
            kdes: match *&builder.kdes {
                Some(kdes) => kdes,
                None => {
                    return Err(MissingPropertyError::from("kdes"));
                }
            },
            kdet: match *&builder.kdet {
                Some(kdet) => kdet,
                None => return Err(MissingPropertyError::from("kdet")),
            },
            k1: match *&builder.k1 {
                Some(k1) => k1,
                None => return Err(MissingPropertyError::from("k1")),
            },
            bf_2tf: match *&builder.bf_2tf {
                Some(bf_2tf) => bf_2tf,
                None => return Err(MissingPropertyError::from("bf/2tf")),
            },
            h_tw: match *&builder.h_tw {
                Some(h_tw) => h_tw,
                None => return Err(MissingPropertyError::from("h/tw")),
            },
            ix: match &builder.ix {
                Some(ix) => *ix,
                None => {
                    return Err(MissingPropertyError::from("Ix"));
                }
            },
            zx: match &builder.zx {
                Some(zx) => *zx,
                None => {
                    return Err(MissingPropertyError::from("Zx"));
                }
            },
            sx: match &builder.sx {
                Some(sx) => *sx,
                None => {
                    return Err(MissingPropertyError::from("Sx"));
                }
            },
            rx: match &builder.rx {
                Some(rx) => *rx,
                None => {
                    return Err(MissingPropertyError::from("Rx"));
                }
            },
            iy: match &builder.iy {
                Some(iy) => *iy,
                None => {
                    return Err(MissingPropertyError::from("Iy"));
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
            j_upper: match *&builder.j_upper {
                Some(j_upper) => j_upper,
                None => return Err(MissingPropertyError::from("J")),
            },
            cw: match *&builder.cw {
                Some(cw) => cw,
                None => {
                    return Err(MissingPropertyError::from("Cw"));
                }
            },
            wno: match *&builder.wno {
                Some(wno) => wno,
                None => return Err(MissingPropertyError::from("Wno")),
            },
            sw1: match *&builder.sw1 {
                Some(sw1) => sw1,
                None => {
                    return Err(MissingPropertyError::from("Sw1"));
                }
            },
            qf: match *&builder.qf {
                Some(qf) => qf,
                None => return Err(MissingPropertyError::from("Qf")),
            },
            qw: match *&builder.qw {
                Some(qw) => qw,
                None => return Err(MissingPropertyError::from("Qw")),
            },
            rts: match *&builder.rts {
                Some(rts) => rts,
                None => return Err(MissingPropertyError::from("rts")),
            },
            ho: match *&builder.ho {
                Some(ho) => ho,
                None => {
                    return Err(MissingPropertyError::from("ho"));
                }
            },
            pa: match *&builder.pa {
                Some(pa) => pa,
                None => return Err(MissingPropertyError::from("PA")),
            },
            pb: match *&builder.pb {
                Some(pb) => pb,
                None => return Err(MissingPropertyError::from("PB")),
            },
            pc: match *&builder.pc {
                Some(pc) => pc,
                None => return Err(MissingPropertyError::from("PC")),
            },
            pd: match *&builder.pd {
                Some(pd) => pd,
                None => return Err(MissingPropertyError::from("PD")),
            },
            t: match *&builder.t {
                Some(t) => t,
                None => return Err(MissingPropertyError::from("T")),
            },
            wgi: match *&builder.wgi {
                Some(wgi) => wgi,
                None => return Err(MissingPropertyError::from("WGi")),
            },
            wgo: *&builder.wgo,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aisc_shapes::shape_builder::ShapeBuilder;

    #[test]
    fn builder_happy_path_works() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("W6X9")
            .with_aisc_manual_label("W6X9")
            .with_t_f(false)
            .with_w_upper(9.0)
            .with_a_upper(2.68)
            .with_d_lower(5.9)
            .with_ddet(5.875)
            .with_bf(3.94)
            .with_bfdet(4.0)
            .with_tw(0.17)
            .with_twdet(0.1875)
            .with_twdet_2(0.125)
            .with_tf(0.215)
            .with_tfdet(0.1875)
            .with_kdes(0.465)
            .with_kdet(0.6875)
            .with_k1(0.5)
            .with_bf_2tf(9.16)
            .with_h_tw(29.2)
            .with_ix(16.4)
            .with_zx(6.23)
            .with_sx(5.56)
            .with_rx(2.47)
            .with_iy(2.2)
            .with_zy(1.72)
            .with_sy(1.11)
            .with_ry(0.905)
            .with_j_upper(0.0405)
            .with_cw(17.7)
            .with_wno(5.6)
            .with_sw1(1.19)
            .with_qf(1.15)
            .with_qw(3.04)
            .with_rts(1.06)
            .with_ho(5.69)
            .with_pa(22.9)
            .with_pb(26.8)
            .with_pc(15.7)
            .with_pd(19.7)
            .with_t(4.5)
            .with_wgi(2.25)
            .try_build::<WideFlange>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!("W6X9", shape.edi_std_nomenclature);
        assert_eq!("W6X9", shape.aisc_manual_label);
        assert!(!shape.t_f);
        assert_eq!(9.0, shape.w_upper);
        assert_eq!(2.68, shape.a_upper);
        assert_eq!(5.9, shape.d_lower);
        assert_eq!(5.875, shape.ddet);
        assert_eq!(3.94, shape.bf);
        assert_eq!(4.0, shape.bfdet);
        assert_eq!(0.17, shape.tw);
        assert_eq!(0.1875, shape.twdet);
        assert_eq!(0.125, shape.twdet_2);
        assert_eq!(0.215, shape.tf);
        assert_eq!(0.1875, shape.tfdet);
        assert_eq!(0.465, shape.kdes);
        assert_eq!(0.6875, shape.kdet);
        assert_eq!(0.5, shape.k1);
        assert_eq!(9.16, shape.bf_2tf);
        assert_eq!(29.2, shape.h_tw);
        assert_eq!(16.4, shape.ix);
        assert_eq!(6.23, shape.zx);
        assert_eq!(5.56, shape.sx);
        assert_eq!(2.47, shape.rx);
        assert_eq!(2.2, shape.iy);
        assert_eq!(1.72, shape.zy);
        assert_eq!(1.11, shape.sy);
        assert_eq!(0.905, shape.ry);
        assert_eq!(0.0405, shape.j_upper);
        assert_eq!(5.6, shape.wno);
        assert_eq!(1.19, shape.sw1);
        assert_eq!(1.15, shape.qf);
        assert_eq!(3.04, shape.qw);
        assert_eq!(1.06, shape.rts);
        assert_eq!(5.69, shape.ho);
        assert_eq!(22.9, shape.pa);
        assert_eq!(26.8, shape.pb);
        assert_eq!(15.7, shape.pc);
        assert_eq!(19.7, shape.pd);
        assert_eq!(4.5, shape.t);
        assert_eq!(2.25, shape.wgi);
    }

    #[test]
    fn missing_edi_std_nom_throws_error() {
        let shape_result = ShapeBuilder::new()
            .with_aisc_manual_label("W6X9")
            .with_t_f(false)
            .with_w_upper(9.0)
            .with_a_upper(2.68)
            .with_d_lower(5.9)
            .with_ddet(5.875)
            .with_bf(3.94)
            .with_bfdet(4.0)
            .with_tw(0.17)
            .with_twdet(0.1875)
            .with_twdet_2(0.125)
            .with_tf(0.215)
            .with_tfdet(0.1875)
            .with_kdes(0.465)
            .with_kdet(0.6875)
            .with_k1(0.5)
            .with_bf_2tf(9.16)
            .with_h_tw(29.2)
            .with_ix(16.4)
            .with_zx(6.23)
            .with_sx(5.56)
            .with_rx(2.47)
            .with_iy(2.2)
            .with_zy(1.72)
            .with_sy(1.11)
            .with_ry(0.905)
            .with_j_upper(0.0405)
            .with_cw(17.7)
            .with_wno(5.6)
            .with_sw1(1.19)
            .with_qf(1.15)
            .with_qw(3.04)
            .with_rts(1.06)
            .with_ho(5.69)
            .with_pa(22.9)
            .with_pb(26.8)
            .with_pc(15.7)
            .with_pd(19.7)
            .with_t(0.5)
            .with_wgi(2.25)
            .try_build::<WideFlange>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property EDI Std Nomenclature was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_aisc_man_lbl_throws_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("W6X9")
            .with_t_f(false)
            .with_w_upper(9.0)
            .with_a_upper(2.68)
            .with_d_lower(5.9)
            .with_ddet(5.875)
            .with_bf(3.94)
            .with_bfdet(4.0)
            .with_tw(0.17)
            .with_twdet(0.1875)
            .with_twdet_2(0.125)
            .with_tf(0.215)
            .with_tfdet(0.1875)
            .with_kdes(0.465)
            .with_kdet(0.6875)
            .with_k1(0.5)
            .with_bf_2tf(9.16)
            .with_h_tw(29.2)
            .with_ix(16.4)
            .with_zx(6.23)
            .with_sx(5.56)
            .with_rx(2.47)
            .with_iy(2.2)
            .with_zy(1.72)
            .with_sy(1.11)
            .with_ry(0.905)
            .with_j_upper(0.0405)
            .with_cw(17.7)
            .with_wno(5.6)
            .with_sw1(1.19)
            .with_qf(1.15)
            .with_qw(3.04)
            .with_rts(1.06)
            .with_ho(5.69)
            .with_pa(22.9)
            .with_pb(26.8)
            .with_pc(15.7)
            .with_pd(19.7)
            .with_t(0.5)
            .with_wgi(2.25)
            .try_build::<WideFlange>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property AISC Manual Label was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
