use serde::Serialize;
use shapes::aisc_shapes::DoubleAngle as AISCDoubleAngle;

/// A data transfer object for double angle (2L) steel profiles
#[derive(Debug, Clone, Serialize)]
pub struct DoubleAngle {
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

impl From<&AISCDoubleAngle> for DoubleAngle {
    fn from(double_angle: &AISCDoubleAngle) -> Self {
        DoubleAngle {
            edi_std_nomenclature: double_angle.edi_std_nomenclature.clone(),
            aisc_manual_label: double_angle.aisc_manual_label.clone(),
            w_upper: double_angle.w_upper,
            a_upper: double_angle.a_upper,
            d_lower: double_angle.d_lower,
            b_lower: double_angle.b_lower,
            t_lower: double_angle.t_lower,
            y_lower: double_angle.y_lower,
            yp: double_angle.yp,
            b_t: double_angle.b_t,
            ix: double_angle.ix,
            zx: double_angle.zx,
            sx: double_angle.sx,
            rx: double_angle.rx,
            iy: double_angle.iy,
            zy: double_angle.zy,
            sy: double_angle.sy,
            ry: double_angle.ry,
            ro: double_angle.ro,
            h_upper: double_angle.h_upper,
        }
    }
}
