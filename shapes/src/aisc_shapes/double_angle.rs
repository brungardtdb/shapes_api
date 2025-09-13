use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(dead_code)]
/// A struct that models the data for cee channel (C) steel profiles
pub struct DoubleAngle<'std_nom, 'aisc_label> {
    /// The shape designation according to the AISC Naming Convention
    /// for Structural Steel Products for Use in Electronic Data Interchange (EDI), June 25, 2001.
    /// This information is intended solely for the use of software developers to facilitate the electronic
    /// labeling of shape-specific data and electronic transfer of that data.
    pub edi_std_nomenclature: &'std_nom str,
    /// The shape designation as seen in the AISC Steel Construction Manual, 16th Edition.
    pub aisc_manual_label: &'aisc_label str,
    /// (W) Nominal weight, lb/ft (kg/m)
    pub w_upper: f64,
    /// (A) Cross-sectional area, in.2 (mm2)
    pub a_upper: f64,
    /// (d) Overall depth of member, or width of shorter leg for DoubleAngles,
    /// or width of the outstanding legs of long legs back-to-back double DoubleAngles,
    /// or the width of the back-to-back legs of short legs back-to-back double DoubleAngles, in. (mm)
    pub d_lower: f64,
    /// Width of the flat wall of square HSS or the shorter flat wall of rectangular HSS,
    /// or width of the longer leg for DoubleAngles,
    /// or width of the back-to-back legs of long legs back-to-back double DoubleAngles,
    /// or width of the outstanding legs of short legs back-to-back double DoubleAngles, in. (mm)
    pub b_lower: f64,
    /// Thickness of DoubleAngle leg, in. (mm)
    pub t_lower: f64,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub y_lower: f64,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub yp: f64,
    /// (b/t) Slenderness ratio for DoubleAngles and channel flange
    pub b_t: f64,
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
    /// Radius of gyration about the y-axis (with no separation for double DoubleAngles back-to-back), in. (mm)
    pub ry: f64,
    /// Polar radius of gyration about the shear center, in. (mm)
    pub ro: f64,
    /// (H) Flexural constant
    pub h_upper: f64,
}

impl<'std_nom, 'aisc_label> TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>
    for DoubleAngle<'std_nom, 'aisc_label>
{
    type Error = MissingPropertyError;
    fn try_from(
        builder: ShapeBuilder<'std_nom, 'aisc_label>,
    ) -> Result<Self, MissingPropertyError> {
        Ok(DoubleAngle {
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
            b_lower: match *&builder.b_lower {
                Some(b_lower) => b_lower,
                None => {
                    return Err(MissingPropertyError::from("b"));
                }
            },
            t_lower: match *&builder.t_lower {
                Some(t_lower) => t_lower,
                None => return Err(MissingPropertyError::from("t")),
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
            b_t: match *&builder.b_t {
                Some(b_t) => b_t,
                None => return Err(MissingPropertyError::from("b/t")),
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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!("2L5X5X3/4", shape.edi_std_nomenclature);
        assert_eq!("2L5X5X3/4", shape.aisc_manual_label);
        assert_eq!(47.2, shape.w_upper);
        assert_eq!(14.0, shape.a_upper);
        assert_eq!(5.0, shape.d_lower);
        assert_eq!(5.0, shape.b_lower);
        assert_eq!(1.38, shape.t_lower);
        assert_eq!(3.5, shape.y_lower);
        assert_eq!(1.3, shape.yp);
        assert_eq!(8.7, shape.b_t);
        assert_eq!(826.0, shape.ix);
        assert_eq!(176.0, shape.zx);
        assert_eq!(97.2, shape.sx);
        assert_eq!(3.64, shape.rx);
        assert_eq!(1590.0, shape.iy);
        assert_eq!(218.0, shape.zy);
        assert_eq!(133.0, shape.sy);
        assert_eq!(5.06, shape.ry);
        assert_eq!(6.84, shape.ro);
        assert_eq!(0.831, shape.h_upper);
    }

    #[test]
    fn missing_edi_std_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property d was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_b_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property b was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_t_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property t was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_y_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            dbg!(&msg);
            assert!("The required property yp was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_b_t_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property b/t was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ix_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ro(6.84)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ry was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ro_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_h_upper(0.831)
            .try_build::<DoubleAngle>();

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
            .with_edi_std_nomenclature("2L5X5X3/4")
            .with_aisc_manual_label("2L5X5X3/4")
            .with_w_upper(47.2)
            .with_a_upper(14.0)
            .with_d_lower(5.0)
            .with_b_lower(5.0)
            .with_t_lower(1.38)
            .with_y_lower(3.50)
            .with_yp(1.3)
            .with_b_t(8.7)
            .with_ix(826.0)
            .with_zx(176.0)
            .with_sx(97.2)
            .with_rx(3.64)
            .with_iy(1590.0)
            .with_zy(218.0)
            .with_sy(133.0)
            .with_ry(5.06)
            .with_ro(6.84)
            .try_build::<DoubleAngle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property H was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
