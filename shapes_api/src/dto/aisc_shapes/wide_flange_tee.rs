use serde::{Deserialize, Serialize};
use shapes::aisc_shapes::WideFlangeTee as WFTee;

/// A data transfer object for wide flange tee (WT) steel profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<&WFTee> for WideFlangeTee {
    fn from(wt: &WFTee) -> Self {
        WideFlangeTee {
            edi_std_nomenclature: wt.edi_std_nomenclature.clone(),
            aisc_manual_label: wt.aisc_manual_label.clone(),
            t_f: wt.t_f,
            w_upper: wt.w_upper,
            a_upper: wt.a_upper,
            d_lower: wt.d_lower,
            ddet: wt.ddet,
            bf: wt.bf,
            bfdet: wt.bfdet,
            tw: wt.tw,
            twdet: wt.twdet,
            twdet_2: wt.twdet_2,
            tf: wt.tf,
            tfdet: wt.tfdet,
            kdes: wt.kdes,
            kdet: wt.kdet,
            y_lower: wt.y_lower,
            yp: wt.yp,
            bf_2tf: wt.bf_2tf,
            d_t: wt.d_t,
            ix: wt.ix,
            zx: wt.zx,
            sx: wt.sx,
            rx: wt.rx,
            iy: wt.iy,
            zy: wt.zy,
            sy: wt.sy,
            ry: wt.ry,
            j_upper: wt.j_upper,
            cw: wt.cw,
            ro: wt.ro,
            h_upper: wt.h_upper,
            pa: wt.pa,
            pb: wt.pb,
            pc: wt.pc,
            pd: wt.pd,
            wgi: wt.wgi,
            wgo: wt.wgo,
        }
    }
}
