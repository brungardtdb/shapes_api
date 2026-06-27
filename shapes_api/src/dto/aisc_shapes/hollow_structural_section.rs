use serde::{Deserialize, Serialize};
use shapes::aisc_shapes::HollowStructuralSection as HSS;

/// A data transfer object for square and rectangular HSS steel profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HollowStructuralSection {
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

impl From<&HSS> for HollowStructuralSection {
    fn from(hss: &HSS) -> Self {
        HollowStructuralSection {
            edi_std_nomenclature: hss.edi_std_nomenclature.clone(),
            aisc_manual_label: hss.aisc_manual_label.clone(),
            w_upper: hss.w_upper,
            a_upper: hss.a_upper,
            ht: hss.ht,
            h: hss.h,
            b_upper: hss.b_upper,
            b_lower: hss.b_lower,
            t_nom: hss.t_nom,
            tdes: hss.tdes,
            b_tdes: hss.b_tdes,
            h_tdes: hss.h_tdes,
            ix: hss.ix,
            zx: hss.zx,
            sx: hss.sx,
            rx: hss.rx,
            iy: hss.iy,
            zy: hss.zy,
            sy: hss.sy,
            ry: hss.ry,
            j_upper: hss.j_upper,
            c_upper: hss.c_upper,
        }
    }
}
