use serde::Serialize;
use shapes::aisc_shapes::Angle as AISCAngle;

/// A data transfer object for angle (L) steel profiles
#[derive(Debug, Clone, Serialize)]
pub struct Angle {
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

impl From<&AISCAngle> for Angle {
    fn from(angle: &AISCAngle) -> Self {
        Angle {
            edi_std_nomenclature: angle.edi_std_nomenclature.clone(),
            aisc_manual_label: angle.aisc_manual_label.clone(),
            w_upper: angle.w_upper,
            a_upper: angle.a_upper,
            d_lower: angle.d_lower,
            b_lower: angle.b_lower,
            t_lower: angle.t_lower,
            kdes: angle.kdes,
            kdet: angle.kdet,
            x_lower: angle.x_lower,
            y_lower: angle.y_lower,
            xp: angle.xp,
            yp: angle.yp,
            b_t: angle.b_t,
            ix: angle.ix,
            zx: angle.zx,
            sx: angle.sx,
            rx: angle.rx,
            iy: angle.iy,
            zy: angle.zy,
            sy: angle.sy,
            ry: angle.ry,
            iz: angle.iz,
            rz: angle.rz,
            sz: angle.sz,
            j_upper: angle.j_upper,
            cw: angle.cw,
            ro: angle.ro,
            h_upper: angle.h_upper,
            tan_a: angle.tan_a,
            iw: angle.iw,
            za: angle.za,
            zb: angle.zb,
            zc: angle.zc,
            wa: angle.wa,
            wb: angle.wb,
            wc: angle.wc,
            swa: angle.swa,
            swb: angle.swb,
            swc: angle.swc,
            sza: angle.sza,
            szb: angle.szb,
            szc: angle.szc,
            pa: angle.pa,
            pa_2: angle.pa_2,
            pb: angle.pb,
        }
    }
}
