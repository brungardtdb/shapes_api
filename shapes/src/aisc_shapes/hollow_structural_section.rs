use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(dead_code)]

/// A struct that models the data for square and rectangular HSS steel profiles
pub struct HollowStructuralSection<'std_nom, 'aisc_label> {
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
    /// (Ht) Overall depth of square HSS or longer wall of rectangular HSS, in. (mm)
    pub ht: f64,
    /// Depth of the flat wall of square HSS or longer flat wall of rectangular HSS, in. (mm)
    pub h: f64,
    /// (B) Overall width of square HSS or shorter wall of rectangular HSS, in. (mm)
    pub b_upper: f64,
    /// (b) Width of the flat wall of square HSS or the shorter flat wall of rectangular HSS,
    /// or width of the longer leg for angles, or width of the back-to-back legs of long legs back-to-back double angles,
    /// or width of the outstanding legs of short legs back-to-back double angles, in. (mm)
    pub b_lower: f64,
    /// Nominal thickness of HSS and pipe wall, in. (mm)
    pub t_nom: f64,
    /// Design thickness of HSS and pipe wall, in. (mm)
    pub tdes: f64,
    /// Distance from outer face of flange to web toe of fillet used for design, in. (mm)
    /// (b/tdes) Slenderness ratio for square HSS or shorter wall of rectangular HSS
    pub b_tdes: f64,
    /// (h/tdes) Slenderness ratio for square HSS or longer wall of rectangular HSS
    pub h_tdes: f64,
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
    /// (C) HSS torsional constant, in.3 (´103 mm3)
    pub c_upper: f64,
}

impl<'std_nom, 'aisc_label> TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>
    for HollowStructuralSection<'std_nom, 'aisc_label>
{
    type Error = MissingPropertyError;
    fn try_from(
        builder: ShapeBuilder<'std_nom, 'aisc_label>,
    ) -> Result<Self, MissingPropertyError> {
        Ok(HollowStructuralSection {
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
            ht: match *&builder.ht {
                Some(ht) => ht,
                None => {
                    return Err(MissingPropertyError::from("Ht"));
                }
            },
            h: match *&builder.h {
                Some(h) => h,
                None => {
                    return Err(MissingPropertyError::from("h"));
                }
            },
            b_upper: match *&builder.b_upper {
                Some(b_upper) => b_upper,
                None => {
                    return Err(MissingPropertyError::from("B"));
                }
            },
            b_lower: match *&builder.b_lower {
                Some(b_lower) => b_lower,
                None => {
                    return Err(MissingPropertyError::from("b"));
                }
            },
            t_nom: match *&builder.t_nom {
                Some(t_nom) => t_nom,
                None => {
                    return Err(MissingPropertyError::from("tnom"));
                }
            },
            tdes: match *&builder.tdes {
                Some(tdes) => tdes,
                None => {
                    return Err(MissingPropertyError::from("tdes"));
                }
            },
            b_tdes: match *&builder.b_tdes {
                Some(b_tdes) => b_tdes,
                None => {
                    return Err(MissingPropertyError::from("b/tdes"));
                }
            },
            h_tdes: match *&builder.h_tdes {
                Some(h_tdes) => h_tdes,
                None => {
                    return Err(MissingPropertyError::from("h/tdes"));
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
            c_upper: match *&builder.c_upper {
                Some(c_upper) => c_upper,
                None => return Err(MissingPropertyError::from("C")),
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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!("HSS8X6X.500", shape.edi_std_nomenclature);
        assert_eq!("HSS8X6X1/2", shape.aisc_manual_label);
        assert_eq!(42.05, shape.w_upper);
        assert_eq!(11.6, shape.a_upper);
        assert_eq!(8.0, shape.ht);
        assert_eq!(6.6, shape.h);
        assert_eq!(6.0, shape.b_upper);
        assert_eq!(4.61, shape.b_lower);
        assert_eq!(0.5, shape.t_nom);
        assert_eq!(0.465, shape.tdes);
        assert_eq!(9.9, shape.b_tdes);
        assert_eq!(14.2, shape.h_tdes);
        assert_eq!(98.2, shape.ix);
        assert_eq!(30.5, shape.zx);
        assert_eq!(24.6, shape.sx);
        assert_eq!(2.91, shape.rx);
        assert_eq!(62.5, shape.iy);
        assert_eq!(24.9, shape.zy);
        assert_eq!(20.8, shape.sy);
        assert_eq!(2.32, shape.ry);
        assert_eq!(127.0, shape.j_upper);
        assert_eq!(38.4, shape.c_upper);
    }

    #[test]
    fn missing_edi_std_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property A was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ht_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Ht was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_h_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property h was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_b_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property B was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_b_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property b was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_t_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property tnom was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_tdes_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property tdes was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_b_tdes_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property b/tdes was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_h_tdes_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property h/tdes was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ix_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_j_upper(127.0)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

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
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_c_upper(38.4)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property J was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_c_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("HSS8X6X.500")
            .with_aisc_manual_label("HSS8X6X1/2")
            .with_w_upper(42.05)
            .with_a_upper(11.6)
            .with_ht(8.0)
            .with_h(6.6)
            .with_b_upper(6.0)
            .with_b_lower(4.61)
            .with_t_nom(0.5)
            .with_tdes(0.465)
            .with_b_tdes(9.9)
            .with_h_tdes(14.2)
            .with_ix(98.2)
            .with_zx(30.5)
            .with_sx(24.6)
            .with_rx(2.91)
            .with_iy(62.5)
            .with_zy(24.9)
            .with_sy(20.8)
            .with_ry(2.32)
            .with_j_upper(127.0)
            .try_build::<HollowStructuralSection>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property C was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
