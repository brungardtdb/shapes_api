use serde::{Deserialize, Serialize};
use shapes::aisc_shapes::WideFlange as WF;

/// A data transfer object for wide flange steel profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WideFlange {
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
    /// Distance from web center line to flange toe of fillet used for detailing, in. (mm)
    pub k1: f64,
    /// (bf/2tf) Slenderness ratio for flange
    pub bf_2tf: f64,
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
    /// (Qf) Statical moment for a point in the flange directly above the vertical edge of the web,
    /// as used in AISC Design Guide 9, in.3 (´103 mm3)
    pub qf: f64,
    /// (Qw) Statical moment for a point at mid-depth of the cross section,
    /// as used in AISC Design Guide 9, in.3 (´103 mm3)
    pub qw: f64,
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
    pub wgi: f64,
    /// (WGo) The bolt spacing between inner and outer fastener holes when the workable gage is compatible with four holes across the flange. See AISC Manual Part 1 for additional information, in. (mm)
    pub wgo: Option<f64>,
}

impl From<&WF> for WideFlange {
    fn from(wf: &WF) -> Self {
        WideFlange {
            edi_std_nomenclature: wf.edi_std_nomenclature.clone(),
            aisc_manual_label: wf.aisc_manual_label.clone(),
            t_f: wf.t_f,
            w_upper: wf.w_upper,
            a_upper: wf.a_upper,
            d_lower: wf.d_lower,
            ddet: wf.ddet,
            bf: wf.bf,
            bfdet: wf.bfdet,
            tw: wf.tw,
            twdet: wf.twdet,
            twdet_2: wf.twdet_2,
            tf: wf.tf,
            tfdet: wf.tfdet,
            kdes: wf.kdes,
            kdet: wf.kdet,
            k1: wf.k1,
            bf_2tf: wf.bf_2tf,
            h_tw: wf.h_tw,
            ix: wf.ix,
            zx: wf.zx,
            sx: wf.sx,
            rx: wf.rx,
            iy: wf.iy,
            zy: wf.zy,
            sy: wf.sy,
            ry: wf.ry,
            j_upper: wf.j_upper,
            cw: wf.cw,
            wno: wf.wno,
            sw1: wf.sw1,
            qf: wf.qf,
            qw: wf.qw,
            rts: wf.rts,
            ho: wf.ho,
            pa: wf.pa,
            pb: wf.pb,
            pc: wf.pc,
            pd: wf.pd,
            t: wf.t,
            wgi: wf.wgi,
            wgo: wf.wgo,
        }
    }
}
