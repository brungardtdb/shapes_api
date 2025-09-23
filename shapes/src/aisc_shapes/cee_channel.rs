use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(dead_code)]
/// A struct that models the data for cee channel (C) steel profiles
pub struct CeeChannel {
    /// The shape designation according to the AISC Naming Convention
    /// for Structural Steel Products for Use in Electronic Data Interchange (EDI), June 25, 2001.
    /// This information is intended solely for the use of software developers to facilitate the electronic
    /// labeling of shape-specific data and electronic transfer of that data.
    pub edi_std_nomenclature: String,
    /// The shape designation as seen in the AISC Steel Construction Manual, 16th Edition.
    pub aisc_manual_label: String,
    /// (W) Nominal weight, lb/ft (kg/m)
    pub w_upper: f64,
    /// (A) Cross-sectional area, in.2 (mm2)
    pub a_upper: f64,
    /// (d) Overall depth of member, or width of shorter leg for angles,
    /// or width of the outstanding legs of long legs back-to-back double angles,
    /// or the width of the back-to-back legs of short legs back-to-back double angles, in. (mm)
    pub d_lower: f64,
    /// Detailing value of member depth, in. (mm)
    pub ddet: f64,
    /// Width of flange, in. (mm)
    pub bf: f64,
    /// Detailing value of flange width, in. (mm)
    pub bfdet: f64,
    /// Thickness of web, in. (mm)
    pub tw: f64,
    ///Detailing value of web thickness, in. (mm)
    pub twdet: f64,
    /// (twdet/2) Detailing value of tw/2, in. (mm)
    pub twdet_2: f64,
    /// Thickness of flange, in. (mm)
    pub tf: f64,
    /// Detailing value of flange thickness, in. (mm)
    pub tfdet: f64,
    /// Distance from outer face of flange to web toe of fillet used for design, in. (mm)
    pub kdes: f64,
    /// Distance from outer face of flange to web toe of fillet used for detailing, in. (mm)
    pub kdet: f64,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub x_lower: f64,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to shear center of member, in. (mm)
    pub eo: f64,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub xp: f64,
    /// (b/t) Slenderness ratio for angles and channel flange
    pub b_t: f64,
    /// (h/tw) Slenderness ratio for web
    pub h_tw: f64,
    /// (Ix) Moment of inertia about the x-axis, in.4 (´106 mm4)
    pub ix: f64,
    /// (Zx) Plastic section modulus about the x-axis, in.3 (´103 mm3)
    pub zx: f64,
    /// (Sx) Elastic section modulus about the x-axis, in.3 (´103 mm3)
    pub sx: f64,
    /// Radius of gyration about the x-axis, in. (mm)
    pub rx: f64,
    /// (Iy) Moment of inertia about the y-axis, in.4 (´106 mm4)
    pub iy: f64,
    /// (Zy) Plastic section modulus about the y-axis, in.3 (´103 mm3)
    pub zy: f64,
    /// (Sy) Elastic section modulus about the y-axis, in.3 (´103 mm3)
    pub sy: f64,
    /// Radius of gyration about the y-axis (with no separation for double angles back-to-back), in. (mm)
    pub ry: f64,
    /// (J) Torsional constant, in.4 (´103 mm4)
    pub j_upper: f64,
    /// (Cw) Warping constant, in.6 (´109 mm6)
    pub cw: f64,
    /// (WNo) Normalized warping function, as used in Design Guide 9, in.2 (mm2)
    pub wno: f64,
    /// (Sw1) Warping statical moment at point 1 on cross section,
    /// as used in AISC Design Guide 9 and shown in Figures 1 and 2, in.4 (´106 mm4)
    pub sw1: f64,
    /// (Sw2) Warping statical moment at point 2 on cross section,
    /// as used in AISC Design Guide 9 and shown in Figure 2, in.4 (´106 mm4)
    pub sw2: f64,
    /// (Sw3) Warping statical moment at point 3 on cross section,
    /// as used in AISC Design Guide 9 and shown in Figure 2, in.4 (´106 mm4)
    pub sw3: f64,
    /// (Qf) Statical moment for a point in the flange directly above the vertical edge of the web,
    /// as used in AISC Design Guide 9, in.3 (´103 mm3)
    pub qf: f64,
    /// (Qw) Statical moment for a point at mid-depth of the cross section,
    /// as used in AISC Design Guide 9, in.3 (´103 mm3)
    pub qw: f64,
    /// Polar radius of gyration about the shear center, in. (mm)
    pub ro: f64,
    /// (H) Flexural constant
    pub h_upper: f64,
    /// Effective radius of gyration, in. (mm)
    pub rts: f64,
    /// Distance between the flange centroids, in. (mm)
    pub ho: f64,
    /// (PA) Shape perimeter minus one flange surface (or short leg surface for a single angle),
    /// as used in Design Guide 19, in. (mm)
    pub pa: f64,
    /// (PB) Shape perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pb: f64,
    /// (PC) Box perimeter minus one flange surface, as used in Design Guide 19, in. (mm)
    pub pc: f64,
    /// (PD) Box perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pd: f64,
    /// (T) Distance between web toes of fillets at top and bottom of web, in. (mm)
    pub t: f64,
    /// (WGi) The workable gage for the inner fastener holes in the flange that provides for entering and tightening clearances and edge distance and spacing requirements.
    /// The actual size, combination, and orientation of fastener components should be compared with the geometry of the cross section to ensure compatibility.
    /// See AISC Manual Part 1 for additional information, in. (mm)
    pub wgi: Option<f64>,
}

impl TryFrom<ShapeBuilder> for CeeChannel {
    type Error = MissingPropertyError;
    fn try_from(builder: ShapeBuilder) -> Result<Self, MissingPropertyError> {
        Ok(CeeChannel {
            edi_std_nomenclature: match &builder.edi_std_nomenclature {
                Some(nom) => nom.to_owned(),
                None => {
                    return Err(MissingPropertyError::from("EDI Std Nomenclature"));
                }
            },
            aisc_manual_label: match &builder.aisc_manual_label {
                Some(label) => label.to_owned(),
                None => {
                    return Err(MissingPropertyError::from("AISC Manual Label"));
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
            x_lower: match *&builder.x_lower {
                Some(x_lower) => x_lower,
                None => {
                    return Err(MissingPropertyError::from("x"));
                }
            },
            eo: match *&builder.eo {
                Some(eo) => eo,
                None => {
                    return Err(MissingPropertyError::from("eo"));
                }
            },
            xp: match *&builder.xp {
                Some(xp) => xp,
                None => {
                    return Err(MissingPropertyError::from("xp"));
                }
            },
            b_t: match *&builder.b_t {
                Some(b_t) => b_t,
                None => return Err(MissingPropertyError::from("b/t")),
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
                    return Err(MissingPropertyError::from("rx"));
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
                    return Err(MissingPropertyError::from("Zy"));
                }
            },
            sy: match &builder.sy {
                Some(sy) => *sy,
                None => {
                    return Err(MissingPropertyError::from("Sy"));
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
            sw2: match *&builder.sw2 {
                Some(sw2) => sw2,
                None => {
                    return Err(MissingPropertyError::from("Sw2"));
                }
            },
            sw3: match *&builder.sw3 {
                Some(sw3) => sw3,
                None => {
                    return Err(MissingPropertyError::from("Sw3"));
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
            ro: match *&builder.ro {
                Some(ro) => ro,
                None => {
                    return Err(MissingPropertyError::from("ro"));
                }
            },
            h_upper: match *&builder.h_upper {
                Some(h_upper) => h_upper,
                None => {
                    return Err(MissingPropertyError::from("H"));
                }
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
            wgi: *&builder.wgi,
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
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!(String::from("C8X11.5"), shape.edi_std_nomenclature);
        assert_eq!(String::from("C8X11.5"), shape.aisc_manual_label);
        assert_eq!(11.5, shape.w_upper);
        assert_eq!(3.37, shape.a_upper);
        assert_eq!(8.0, shape.d_lower);
        assert_eq!(8.0, shape.ddet);
        assert_eq!(2.26, shape.bf);
        assert_eq!(2.25, shape.bfdet);
        assert_eq!(0.22, shape.tw);
        assert_eq!(0.25, shape.twdet);
        assert_eq!(0.125, shape.twdet_2);
        assert_eq!(0.390, shape.tf);
        assert_eq!(0.375, shape.tfdet);
        assert_eq!(0.938, shape.kdes);
        assert_eq!(0.9375, shape.kdet);
        assert_eq!(0.572, shape.x_lower);
        assert_eq!(0.697, shape.eo);
        assert_eq!(0.211, shape.xp);
        assert_eq!(5.79, shape.b_t);
        assert_eq!(28.9, shape.h_tw);
        assert_eq!(32.5, shape.ix);
        assert_eq!(9.63, shape.zx);
        assert_eq!(8.14, shape.sx);
        assert_eq!(3.11, shape.rx);
        assert_eq!(1.31, shape.iy);
        assert_eq!(1.57, shape.zy);
        assert_eq!(0.775, shape.sy);
        assert_eq!(0.623, shape.ry);
        assert_eq!(0.13, shape.j_upper);
        assert_eq!(5.11, shape.wno);
        assert_eq!(1.34, shape.sw1);
        assert_eq!(0.855, shape.sw2);
        assert_eq!(0.430, shape.sw3);
        assert_eq!(3.03, shape.qf);
        assert_eq!(4.79, shape.qw);
        assert_eq!(3.41, shape.ro);
        assert_eq!(0.862, shape.h_upper);
        assert_eq!(0.756, shape.rts);
        assert_eq!(7.61, shape.ho);
        assert_eq!(21.9, shape.pa);
        assert_eq!(24.1, shape.pb);
        assert_eq!(18.3, shape.pc);
        assert_eq!(20.5, shape.pd);
        assert_eq!(6.125, shape.t);
        assert_eq!(1.375, shape.wgi.unwrap());
    }

    #[test]
    fn missing_edi_std_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property EDI Std Nomenclature was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_aisc_man_lbl_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property AISC Manual Label was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_w_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property W was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_a_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property A was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_d_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property d was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ddet_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ddet was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_bf_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property bf was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_bfdet_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property bfdet was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_tw_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property tw was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_twdet_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property twdet was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_twdet_2_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property twdet/2 was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_tf_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property tf was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_tfdet_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property tfdet was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_kdes_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property kdes was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_kdet_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property kdet was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_x_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property x was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_eo_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property eo was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_xp_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property xp was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_b_t_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property b/t was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_h_tw_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property h/tw was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ix_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Ix was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_zx_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Zx was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sx_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Sx was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_rx_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property rx was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_iy_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Iy was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_zy_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Zy was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sy_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Sy was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ry_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ry was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_j_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property J was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_cw_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Cw was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_wno_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Wno was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sw1_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Sw1 was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sw2_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Sw2 was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sw3_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Sw3 was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_qf_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Qf was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_qw_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Qw was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ro_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ro was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_h_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property H was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_rts_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property rts was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ho_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ho was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pa_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PA was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pb_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PB was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pc_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pd(20.5)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PC was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pd_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_t(6.125)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PD was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_t_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("C8X11.5"))
            .with_aisc_manual_label(String::from("C8X11.5"))
            .with_w_upper(11.5)
            .with_a_upper(3.37)
            .with_d_lower(8.0)
            .with_ddet(8.0)
            .with_bf(2.26)
            .with_bfdet(2.25)
            .with_tw(0.22)
            .with_twdet(0.25)
            .with_twdet_2(0.125)
            .with_tf(0.390)
            .with_tfdet(0.375)
            .with_kdes(0.938)
            .with_kdet(0.9375)
            .with_x_lower(0.572) 
            .with_eo(0.697) 
            .with_xp(0.211) 
            .with_b_t(5.79) 
            .with_h_tw(28.9)
            .with_ix(32.5)
            .with_zx(9.63)
            .with_sx(8.14)
            .with_rx(3.11)
            .with_iy(1.31)
            .with_zy(1.57)
            .with_sy(0.775)
            .with_ry(0.623)
            .with_j_upper(0.13)
            .with_cw(16.5)
            .with_wno(5.11)
            .with_sw1(1.34)
            .with_sw2(0.855) 
            .with_sw3(0.430) 
            .with_qf(3.03)
            .with_qw(4.79)
            .with_ro(3.41) 
            .with_h_upper(0.862) 
            .with_rts(0.756)
            .with_ho(7.61)
            .with_pa(21.9)
            .with_pb(24.1)
            .with_pc(18.3)
            .with_pd(20.5)
            .with_wgi(1.375)
            .try_build::<CeeChannel>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property T was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
