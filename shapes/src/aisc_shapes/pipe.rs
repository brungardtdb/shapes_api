use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(dead_code)]

/// A struct that models the data for pipe steel profiles
pub struct Pipe {
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
    /// (OD) Outside diameter of round HSS or pipe, in. (mm)
    pub od: f64,
    /// (ID) Inside diameter of pipe, in. (mm)
    pub id: f64,
    /// Nominal thickness of HSS and pipe wall, in. (mm)
    pub t_nom: f64,
    /// Design thickness of HSS and pipe wall, in. (mm)
    pub tdes: f64,
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
}

impl TryFrom<ShapeBuilder> for Pipe {
    type Error = MissingPropertyError;
    fn try_from(builder: ShapeBuilder) -> Result<Self, MissingPropertyError> {
        Ok(Pipe {
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
            w_upper: match *&builder.w_upper {
                Some(w) => w,
                None => {
                    return Err(MissingPropertyError::from("W"));
                }
            },
            a_upper: match *&builder.a_upper {
                Some(a_upper) => a_upper,
                None => {
                    return Err(MissingPropertyError::from("A"));
                }
            },
            od: match *&builder.od {
                Some(od) => od,
                None => return Err(MissingPropertyError::from("OD")),
            },
            id: match *&builder.id {
                Some(id) => id,
                None => return Err(MissingPropertyError::from("ID")),
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
            d_t: match *&builder.d_t {
                Some(d_t) => d_t,
                None => return Err(MissingPropertyError::from("D/t")),
            },
            ix: match *&builder.ix {
                Some(ix) => ix,
                None => {
                    return Err(MissingPropertyError::from("Ix"));
                }
            },
            zx: match *&builder.zx {
                Some(zx) => zx,
                None => {
                    return Err(MissingPropertyError::from("Zx"));
                }
            },
            sx: match *&builder.sx {
                Some(sx) => sx,
                None => {
                    return Err(MissingPropertyError::from("Sx"));
                }
            },
            rx: match *&builder.rx {
                Some(rx) => rx,
                None => {
                    return Err(MissingPropertyError::from("rx"));
                }
            },
            iy: match *&builder.iy {
                Some(iy) => iy,
                None => {
                    return Err(MissingPropertyError::from("Iy"));
                }
            },
            zy: match *&builder.zy {
                Some(zy) => zy,
                None => {
                    return Err(MissingPropertyError::from("Zy"));
                }
            },
            sy: match *&builder.sy {
                Some(sy) => sy,
                None => {
                    return Err(MissingPropertyError::from("Sy"));
                }
            },
            ry: match *&builder.ry {
                Some(ry) => ry,
                None => {
                    return Err(MissingPropertyError::from("ry"));
                }
            },
            j_upper: match *&builder.j_upper {
                Some(j_upper) => j_upper,
                None => return Err(MissingPropertyError::from("J")),
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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!(String::from("Pipe26STD"), shape.edi_std_nomenclature);
        assert_eq!(String::from("Pipe26STD"), shape.aisc_manual_label);
        assert_eq!(103.0, shape.w_upper);
        assert_eq!(28.2, shape.a_upper);
        assert_eq!(26.0, shape.od);
        assert_eq!(25.3, shape.id);
        assert_eq!(0.375, shape.t_nom);
        assert_eq!(0.349, shape.tdes);
        assert_eq!(74.5, shape.d_t);
        assert_eq!(2320.0, shape.ix);
        assert_eq!(230.0, shape.zx);
        assert_eq!(178.0, shape.sx);
        assert_eq!(9.07, shape.rx);
        assert_eq!(191.0, shape.iy);
        assert_eq!(230.0, shape.zy);
        assert_eq!(178.0, shape.sy);
        assert_eq!(9.07, shape.ry);
        assert_eq!(4640.0, shape.j_upper);
    }

    #[test]
    fn missing_edi_std_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property A was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_od_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property OD was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_id_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ID was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_t_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property tdes was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_d_t_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property D/t was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_ix_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_ry(9.07)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_j_upper(4640.0)
            .try_build::<Pipe>();

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
            .with_edi_std_nomenclature(String::from("Pipe26STD"))
            .with_aisc_manual_label(String::from("Pipe26STD"))
            .with_w_upper(103.0)
            .with_a_upper(28.2)
            .with_od(26.0)
            .with_id(25.3)
            .with_t_nom(0.375)
            .with_tdes(0.349)
            .with_d_t(74.5)
            .with_ix(2320.0)
            .with_zx(230.0)
            .with_sx(178.0)
            .with_rx(9.07)
            .with_iy(191.0)
            .with_zy(230.0)
            .with_sy(178.0)
            .with_ry(9.07)
            .try_build::<Pipe>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property J was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
