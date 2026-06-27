use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// A struct that models the data for wide flange tee (WT) steel profiles
pub struct WideFlangeTee {
    /// The shape designation according to the AISC Naming Convention
    /// for Structural Steel Products for Use in Electronic Data Interchange (EDI), June 25, 2001.
    /// This information is intended solely for the use of software developers to facilitate the electronic
    /// labeling of shape-specific data and electronic transfer of that data.
    pub edi_std_nomenclature: String,
    /// The shape designation as seen in the AISC Steel Construction Manual, 16th Edition.
    pub aisc_manual_label: String,
    /// Boolean variable that indicates whether there is a special note for that shape.
    pub t_f: bool,
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
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub y_lower: f64,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub yp: f64,
    /// (bf/2tf) Slenderness ratio for flange
    pub bf_2tf: f64,
    /// (D/t) Slenderness ratio for round HSS and pipe (D = ID), or tee shapes (D = d)
    pub d_t: f64,
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
    /// Polar radius of gyration about the shear center, in. (mm)
    pub ro: f64,
    /// (H) Flexural constant
    pub h_upper: f64,
    /// (PA) Shape perimeter minus one flange surface (or short leg surface for a single angle),
    /// as used in Design Guide 19, in. (mm)
    pub pa: f64,
    /// (PB) Shape perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pb: f64,
    /// (PC) Box perimeter minus one flange surface, as used in Design Guide 19, in. (mm)
    pub pc: f64,
    /// (PD) Box perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pd: f64,
    /// (WGi) The workable gage for the inner fastener holes in the flange that provides for entering and tightening clearances and edge distance and spacing requirements.
    /// The actual size, combination, and orientation of fastener components should be compared with the geometry of the cross section to ensure compatibility.
    /// See AISC Manual Part 1 for additional information, in. (mm)
    pub wgi: f64,
    /// (WGo) The bolt spacing between inner and outer fastener holes when the workable gage is compatible with four holes across the flange. See AISC Manual Part 1 for additional information, in. (mm)
    pub wgo: Option<f64>,
}

impl TryFrom<ShapeBuilder> for WideFlangeTee {
    type Error = MissingPropertyError;
    fn try_from(builder: ShapeBuilder) -> Result<Self, MissingPropertyError> {
        Ok(WideFlangeTee {
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
            y_lower: match *&builder.y_lower {
                Some(y_lower) => y_lower,
                None => {
                    return Err(MissingPropertyError::from("y"));
                }
            },
            yp: match *&builder.yp {
                Some(yp) => yp,
                None => {
                    return Err(MissingPropertyError::from("yp"));
                }
            },
            bf_2tf: match *&builder.bf_2tf {
                Some(bf_2tf) => bf_2tf,
                None => return Err(MissingPropertyError::from("bf/2tf")),
            },
            d_t: match *&builder.d_t {
                Some(d_t) => d_t,
                None => {
                    return Err(MissingPropertyError::from("d/t"));
                }
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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!(String::from("WT12X31"), shape.edi_std_nomenclature);
        assert_eq!(String::from("WT12X31"), shape.aisc_manual_label);
        assert!(!shape.t_f);
        assert_eq!(31.0, shape.w_upper);
        assert_eq!(9.11, shape.a_upper);
        assert_eq!(11.9, shape.d_lower);
        assert_eq!(11.875, shape.ddet);
        assert_eq!(7.04, shape.bf);
        assert_eq!(7.0, shape.bfdet);
        assert_eq!(0.43, shape.tw);
        assert_eq!(0.4375, shape.twdet);
        assert_eq!(0.25, shape.twdet_2);
        assert_eq!(0.59, shape.tf);
        assert_eq!(0.5625, shape.tfdet);
        assert_eq!(1.09, shape.kdes);
        assert_eq!(1.5, shape.kdet);
        assert_eq!(3.46, shape.y_lower);
        assert_eq!(1.28, shape.yp);
        assert_eq!(5.97, shape.bf_2tf);
        assert_eq!(27.7, shape.d_t);
        assert_eq!(131.0, shape.ix);
        assert_eq!(28.4, shape.zx);
        assert_eq!(15.6, shape.sx);
        assert_eq!(3.79, shape.rx);
        assert_eq!(17.2, shape.iy);
        assert_eq!(7.85, shape.zy);
        assert_eq!(4.9, shape.sy);
        assert_eq!(1.38, shape.ry);
        assert_eq!(0.85, shape.j_upper);
        assert_eq!(3.92, shape.cw);
        assert_eq!(5.13, shape.ro);
        assert_eq!(0.619, shape.h_upper);
        assert_eq!(30.2, shape.pa);
        assert_eq!(37.2, shape.pb);
        assert_eq!(30.8, shape.pc);
        assert_eq!(37.9, shape.pd);
        assert_eq!(2.25, shape.wgi);
    }

    #[test]
    fn missing_edi_std_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property AISC Manual Label was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_t_f_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property T_F was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_w_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property kdet was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_y_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property y was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_yp_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property yp was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_bf_2tf_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property bf/2tf was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_d_t_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property d/t was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ix_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Cw was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ro_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property H was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pa_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pc(30.8)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pd(37.9)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

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
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .wgi(2.25)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PD was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_wgi_returns_error() {
        let shape_result = ShapeBuilder::new()
            .edi_std_nomenclature(String::from("WT12X31"))
            .aisc_manual_label(String::from("WT12X31"))
            .t_f(false)
            .w_upper(31.0)
            .a_upper(9.11)
            .d_lower(11.9)
            .ddet(11.875)
            .bf(7.04)
            .bfdet(7.0)
            .tw(0.43)
            .twdet(0.4375)
            .twdet_2(0.25)
            .tf(0.59)
            .tfdet(0.5625)
            .kdes(1.09)
            .kdet(1.5)
            .y_lower(3.46)
            .yp(1.28)
            .bf_2tf(5.97)
            .d_t(27.7)
            .ix(131.0)
            .zx(28.4)
            .sx(15.6)
            .rx(3.79)
            .iy(17.2)
            .zy(7.85)
            .sy(4.9)
            .ry(1.38)
            .j_upper(0.85)
            .cw(3.92)
            .ro(5.13)
            .h_upper(0.619)
            .pa(30.2)
            .pb(37.2)
            .pc(30.8)
            .pd(37.9)
            .try_build::<WideFlangeTee>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property WGi was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
