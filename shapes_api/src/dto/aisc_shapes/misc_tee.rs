use serde::Serialize;
use shapes::aisc_shapes::MiscTee as AISCMiscTee;

/// A data transfer object for misc. tee (MT) steel profiles
#[derive(Debug, Clone, Serialize)]
pub struct MiscTee {
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
    /// (WGi) The workable gage for the inner fastener holes in the flange that provides for entering and tightening clearances and edge distance and spacing requirements.
    /// The actual size, combination, and orientation of fastener components should be compared with the geometry of the cross section to ensure compatibility.
    /// See AISC Manual Part 1 for additional information, in. (mm)
    pub wgi: Option<f64>,
}

impl From<&AISCMiscTee> for MiscTee {
    fn from(misc_tee: &AISCMiscTee) -> Self {
        MiscTee {
            edi_std_nomenclature: misc_tee.edi_std_nomenclature.clone(),
            aisc_manual_label: misc_tee.aisc_manual_label.clone(),
            t_f: misc_tee.t_f,
            w_upper: misc_tee.w_upper,
            a_upper: misc_tee.a_upper,
            d_lower: misc_tee.d_lower,
            ddet: misc_tee.ddet,
            bf: misc_tee.bf,
            bfdet: misc_tee.bfdet,
            tw: misc_tee.tw,
            twdet: misc_tee.twdet,
            twdet_2: misc_tee.twdet_2,
            tf: misc_tee.tf,
            tfdet: misc_tee.tfdet,
            kdes: misc_tee.kdes,
            kdet: misc_tee.kdet,
            y_lower: misc_tee.y_lower,
            yp: misc_tee.yp,
            bf_2tf: misc_tee.bf_2tf,
            d_t: misc_tee.d_t,
            ix: misc_tee.ix,
            zx: misc_tee.zx,
            sx: misc_tee.sx,
            rx: misc_tee.rx,
            iy: misc_tee.iy,
            zy: misc_tee.zy,
            sy: misc_tee.sy,
            ry: misc_tee.ry,
            j_upper: misc_tee.j_upper,
            cw: misc_tee.cw,
            ro: misc_tee.ro,
            h_upper: misc_tee.h_upper,
            wgi: misc_tee.wgi,
        }
    }
}
