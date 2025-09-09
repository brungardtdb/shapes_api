use crate::aisc_shapes::{MissingPropertyError, ShapeBuilder};
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(dead_code)]
/// A struct that models the data for cee channel (C) steel profiles
pub struct Angle<'std_nom, 'aisc_label> {
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
    /// (d) Overall depth of member, or width of shorter leg for angles,
    /// or width of the outstanding legs of long legs back-to-back double angles,
    /// or the width of the back-to-back legs of short legs back-to-back double angles, in. (mm)
    pub d_lower: f64,
    /// Width of the flat wall of square HSS or the shorter flat wall of rectangular HSS,
    /// or width of the longer leg for angles,
    /// or width of the back-to-back legs of long legs back-to-back double angles,
    /// or width of the outstanding legs of short legs back-to-back double angles, in. (mm)
    pub b_lower: f64,
    /// Thickness of angle leg, in. (mm)
    pub t_lower: f64,
    /// Distance from outer face of flange to web toe of fillet used for design, in. (mm)
    pub kdes: f64,
    /// Distance from outer face of flange to web toe of fillet used for detailing, in. (mm)
    pub kdet: f64,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub x_lower: f64,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub y_lower: f64,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub xp: f64,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub yp: f64,
    /// (b/t) Slenderness ratio for angles and channel flange
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
    /// Radius of gyration about the y-axis (with no separation for double angles back-to-back), in. (mm)
    pub ry: f64,
    /// (Iz) Moment of inertia about the z-axis, in.4 (´106 mm4)
    pub iz: f64,
    /// Radius of gyration about the z-axis, in. (mm)
    pub rz: f64,
    /// (Sz) Elastic section modulus about the z-axis, in.3 (´103 mm3). For single angles, see SzA, SzB, and SzC.
    pub sz: f64,
    /// (J) Torsional constant, in.4 (´103 mm4)
    pub j_upper: f64,
    /// (Cw) Warping constant, in.6 (´109 mm6)
    pub cw: f64,
    /// Polar radius of gyration about the shear center, in. (mm)
    pub ro: f64,
    /// (H) Flexural constant
    pub h_upper: Option<f64>,
    /// (tan(α)) Tangent of the angle between the y-y and z-z axes for single angles,
    /// where a is shown in Figure 3
    pub tan_a: f64,
    /// (Iw) Moment of inertia about the w-axis for single angles, in.4 (´106 mm4)
    pub iw: f64,
    /// (zA) Distance from point A to center of gravity along z-axis
    pub za: f64,
    /// (zB) Distance from point B to center of gravity along z-axis
    pub zb: f64,
    /// (zC) Distance from point C to center of gravity along z-axis
    pub zc: f64,
    /// (wA) Distance from point A to center of gravity along w-axis
    pub wa: f64,
    /// (wB) Distance from point B to center of gravity along w-axis
    pub wb: f64,
    /// (wC) Distance from point C to center of gravity along w-axis
    pub wc: f64,
    /// (SwA) Elastic section modulus about the w-axis at point A on cross section
    pub swa: f64,
    /// (SwB) Elastic section modulus about the w-axis at point B on cross section
    pub swb: Option<f64>,
    /// (SwC) Elastic section modulus about the w-axis at point C on cross section
    pub swc: f64,
    /// (SzA) Elastic section modulus about the z-axis at point A on cross section
    pub sza: f64,
    /// (SzB) Elastic section modulus about the z-axis at point B on cross section
    pub szb: f64,
    /// (SzC) Elastic section modulus about the z-axis at point C on cross section
    pub szc: f64,
    /// (PA) Shape perimeter minus one flange surface (or short leg surface for a single angle),
    /// as used in Design Guide 19, in. (mm)
    pub pa: f64,
    /// (PA2) Single angle shape perimeter minus long leg surface,
    /// as used in AISC Design Guide 19, in. (mm)
    pub pa_2: f64,
    /// (PB) Shape perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pb: f64,
}

impl<'std_nom, 'aisc_label> TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>
    for Angle<'std_nom, 'aisc_label>
{
    type Error = MissingPropertyError;
    fn try_from(
        builder: ShapeBuilder<'std_nom, 'aisc_label>,
    ) -> Result<Self, MissingPropertyError> {
        Ok(Angle {
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
            y_lower: match *&builder.y_lower {
                Some(y_lower) => y_lower,
                None => {
                    return Err(MissingPropertyError::from("y"));
                }
            },
            xp: match *&builder.xp {
                Some(xp) => xp,
                None => {
                    return Err(MissingPropertyError::from("xp"));
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
            iz: match &builder.iz {
                Some(iz) => *iz,
                None => {
                    return Err(MissingPropertyError::from("Iz"));
                }
            },
            rz: match &builder.rz {
                Some(rz) => *rz,
                None => {
                    return Err(MissingPropertyError::from("rz"));
                }
            },
            sz: match &builder.sz {
                Some(sz) => *sz,
                None => {
                    return Err(MissingPropertyError::from("Sz"));
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
            h_upper: *&builder.h_upper,
            tan_a: match *&builder.tan_a {
                Some(tan_a) => tan_a,
                None => return Err(MissingPropertyError::from("tan(α)")),
            },
            iw: match *&builder.iw {
                Some(iw) => iw,
                None => {
                    return Err(MissingPropertyError::from("Iw"));
                }
            },
            za: match *&builder.za {
                Some(za) => za,
                None => {
                    return Err(MissingPropertyError::from("zA"));
                }
            },
            zb: match *&builder.zb {
                Some(zb) => zb,
                None => {
                    return Err(MissingPropertyError::from("zB"));
                }
            },
            zc: match *&builder.zc {
                Some(zc) => zc,
                None => {
                    return Err(MissingPropertyError::from("zC"));
                }
            },
            wa: match *&builder.wa {
                Some(wa) => wa,
                None => {
                    return Err(MissingPropertyError::from("wA"));
                }
            },
            wb: match *&builder.wb {
                Some(wb) => wb,
                None => {
                    return Err(MissingPropertyError::from("wB"));
                }
            },
            wc: match *&builder.wc {
                Some(wc) => wc,
                None => {
                    return Err(MissingPropertyError::from("wC"));
                }
            },
            swa: match *&builder.swa {
                Some(swa) => swa,
                None => {
                    return Err(MissingPropertyError::from("SwA"));
                }
            },
            swb: *&builder.swb,
            swc: match *&builder.swc {
                Some(swc) => swc,
                None => {
                    return Err(MissingPropertyError::from("SwC"));
                }
            },
            sza: match *&builder.sza {
                Some(sza) => sza,
                None => {
                    return Err(MissingPropertyError::from("SzA"));
                }
            },
            szb: match *&builder.szb {
                Some(szb) => szb,
                None => {
                    return Err(MissingPropertyError::from("SzB"));
                }
            },
            szc: match *&builder.szc {
                Some(szc) => szc,
                None => {
                    return Err(MissingPropertyError::from("SzC"));
                }
            },
            pa: match *&builder.pa {
                Some(pa) => pa,
                None => return Err(MissingPropertyError::from("PA")),
            },
            pa_2: match *&builder.pa_2 {
                Some(pa_2) => pa_2,
                None => return Err(MissingPropertyError::from("PA2")),
            },
            pb: match *&builder.pb {
                Some(pb) => pb,
                None => return Err(MissingPropertyError::from("PB")),
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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_ok());
        let shape = shape_result.unwrap();
        assert_eq!("L8X6X1/2", shape.edi_std_nomenclature);
        assert_eq!("L8X6X1/2", shape.aisc_manual_label);
        assert_eq!(23.0, shape.w_upper);
        assert_eq!(6.8, shape.a_upper);
        assert_eq!(6.0, shape.d_lower);
        assert_eq!(8.0, shape.b_lower);
        assert_eq!(0.5, shape.t_lower);
        assert_eq!(1.0, shape.kdes);
        assert_eq!(1.0, shape.kdet);
        assert_eq!(1.46, shape.x_lower);
        assert_eq!(2.46, shape.y_lower);
        assert_eq!(0.425, shape.xp);
        assert_eq!(1.2, shape.yp);
        assert_eq!(16.0, shape.b_t);
        assert_eq!(44.4, shape.ix);
        assert_eq!(14.6, shape.zx);
        assert_eq!(8.01, shape.sx);
        assert_eq!(2.55, shape.rx);
        assert_eq!(21.7, shape.iy);
        assert_eq!(8.52, shape.zy);
        assert_eq!(4.79, shape.sy);
        assert_eq!(1.79, shape.ry);
        assert_eq!(0.584, shape.j_upper);
        assert_eq!(2.28, shape.cw);
        assert_eq!(4.01, shape.ro);
        assert_eq!(0.557, shape.tan_a);
        assert_eq!(54.6, shape.iw);
        assert_eq!(4.14, shape.za);
        assert_eq!(1.44, shape.zb);
        assert_eq!(5.41, shape.zc);
        assert_eq!(2.87, shape.wa);
        assert_eq!(2.51, shape.wb);
        assert_eq!(1.62, shape.wc);
        assert_eq!(14.8, shape.swa);
        assert_eq!(42.2, shape.swb.unwrap());
        assert_eq!(11.2, shape.swc);
        assert_eq!(4.46, shape.sza);
        assert_eq!(5.1, shape.szb);
        assert_eq!(7.9, shape.szc);
        assert_eq!(22.0, shape.pa);
        assert_eq!(20.0, shape.pa_2);
        assert_eq!(28.0, shape.pb);
    }

    #[test]
    fn missing_edi_std_nom_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property t was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_kdes_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property x was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_y_lower_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property y was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_xp_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property xp was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_yp_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ry was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_iz_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Iz was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_rz_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property rz was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sz_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Sz was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_j_upper_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

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
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property ro was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_tan_a_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_ro(4.01)
            .with_cw(2.28)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            dbg!(&msg);
            assert!("The required property tan(α) was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_iw_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property Iw was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_za_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property zA was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_zb_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property zB was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_zc_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property zC was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_wa_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property wA was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_wb_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property wB was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_wc_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property wC was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_swa_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property SwA was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_swc_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property SwC was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_sza_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property SzA was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_szb_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property SzB was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_szc_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property SzC was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pa_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa_2(20.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PA was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pa_2_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pb(28.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PA2 was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }

    #[test]
    fn missing_pb_returns_error() {
        let shape_result = ShapeBuilder::new()
            .with_edi_std_nomenclature("L8X6X1/2")
            .with_aisc_manual_label("L8X6X1/2")
            .with_w_upper(23.0)
            .with_a_upper(6.8)
            .with_d_lower(6.0)
            .with_b_lower(8.0)
            .with_t_lower(0.5)
            .with_kdes(1.0)
            .with_kdet(1.0)
            .with_x_lower(1.46)
            .with_y_lower(2.46)
            .with_xp(0.425)
            .with_yp(1.2)
            .with_b_t(16.0)
            .with_ix(44.4)
            .with_zx(14.6)
            .with_sx(8.01)
            .with_rx(2.55)
            .with_iy(21.7)
            .with_zy(8.52)
            .with_sy(4.79)
            .with_ry(1.79)
            .with_iz(11.5)
            .with_rz(1.30)
            .with_sz(3.98)
            .with_j_upper(0.584)
            .with_cw(2.28)
            .with_ro(4.01)
            .with_tan_a(0.557)
            .with_iw(54.6)
            .with_za(4.14)
            .with_zb(1.44)
            .with_zc(5.41)
            .with_wa(2.87)
            .with_wb(2.51)
            .with_wc(1.62)
            .with_swa(14.8)
            .with_swb(42.2)
            .with_swc(11.2)
            .with_sza(4.46)
            .with_szb(5.1)
            .with_szc(7.9)
            .with_pa(22.0)
            .with_pa_2(20.0)
            .try_build::<Angle>();

        assert!(shape_result.is_err());
        if let Err(err) = shape_result {
            let msg = format!("{}", err);
            assert!("The required property PB was missing." == msg);
        } else {
            unreachable!("Failed shape conversion did not return an error");
        }
    }
}
