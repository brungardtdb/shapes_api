use serde::Serialize;
use shapes::aisc_shapes::StructuralBeam as SBeam;

/// A data transfer object for structural beams (S) steel profiles
#[derive(Debug, Clone, Serialize)]
pub struct StructuralBeam {
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
    pub wgi: Option<f64>,
}

impl From<&SBeam> for StructuralBeam {
    fn from(sbeam: &SBeam) -> Self {
        StructuralBeam {
            edi_std_nomenclature: sbeam.edi_std_nomenclature.clone(),
            aisc_manual_label: sbeam.aisc_manual_label.clone(),
            w_upper: sbeam.w_upper,
            a_upper: sbeam.a_upper,
            d_lower: sbeam.d_lower,
            ddet: sbeam.ddet,
            bf: sbeam.bf,
            bfdet: sbeam.bfdet,
            tw: sbeam.tw,
            twdet: sbeam.twdet,
            twdet_2: sbeam.twdet_2,
            tf: sbeam.tf,
            tfdet: sbeam.tfdet,
            kdes: sbeam.kdes,
            kdet: sbeam.kdet,
            bf_2tf: sbeam.bf_2tf,
            h_tw: sbeam.h_tw,
            ix: sbeam.ix,
            zx: sbeam.zx,
            sx: sbeam.sx,
            rx: sbeam.rx,
            iy: sbeam.iy,
            zy: sbeam.zy,
            sy: sbeam.sy,
            ry: sbeam.ry,
            j_upper: sbeam.j_upper,
            cw: sbeam.cw,
            wno: sbeam.wno,
            sw1: sbeam.sw1,
            qf: sbeam.qf,
            qw: sbeam.qw,
            rts: sbeam.rts,
            ho: sbeam.ho,
            pa: sbeam.pa,
            pb: sbeam.pb,
            pc: sbeam.pc,
            pd: sbeam.pd,
            t: sbeam.t,
            wgi: sbeam.wgi,
        }
    }
}
