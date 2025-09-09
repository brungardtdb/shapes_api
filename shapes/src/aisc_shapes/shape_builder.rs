use std::convert::TryFrom;

#[derive(Debug)]
#[allow(dead_code)]
/// [ShapeBuilder] is use to build AISC steel shapes,
/// it contains a superset of the required fields for
/// each shape defined in the AISC steel shapes database.
/// The ['std_nom] and ['aisc_label] lifetimes correspond
/// to the edi_std_nomenclature and aisc_manual_label
/// fields respectively.
pub struct ShapeBuilder<'std_nom, 'aisc_label> {
    /// The shape designation according to the AISC Naming Convention
    /// for Structural Steel Products for Use in Electronic Data Interchange (EDI), June 25, 2001.
    /// This information is intended solely for the use of software developers to facilitate the electronic
    /// labeling of shape-specific data and electronic transfer of that data.
    pub edi_std_nomenclature: Option<&'std_nom str>,
    /// The shape designation as seen in the AISC Steel Construction Manual, 16th Edition.
    pub aisc_manual_label: Option<&'aisc_label str>,
    /// Boolean variable that indicates whether there is a special note for that shape.
    pub t_f: Option<bool>,
    /// (W) Nominal weight, lb/ft (kg/m)
    pub w_upper: Option<f64>,
    /// (A) Cross-sectional area, in.2 (mm2)
    pub a_upper: Option<f64>,
    /// (d) Overall depth of member, or width of shorter leg for angles,
    /// or width of the outstanding legs of long legs back-to-back double angles,
    /// or the width of the back-to-back legs of short legs back-to-back double angles, in. (mm)
    pub d_lower: Option<f64>,
    /// Detailing value of member depth, in. (mm)
    pub ddet: Option<f64>,
    /// (Ht) Overall depth of square HSS or longer wall of rectangular HSS, in. (mm)
    pub ht: Option<f64>,
    /// Depth of the flat wall of square HSS or longer flat wall of rectangular HSS, in. (mm)
    pub h: Option<f64>,
    /// (OD) Outside diameter of round HSS or pipe, in. (mm)
    pub od: Option<f64>,
    /// Width of flange, in. (mm)
    pub bf: Option<f64>,
    /// Detailing value of flange width, in. (mm)
    pub bfdet: Option<f64>,
    /// (B) Overall width of square HSS or shorter wall of rectangular HSS, in. (mm)
    pub b_upper: Option<f64>,
    /// (b) Width of the flat wall of square HSS or the shorter flat wall of rectangular HSS,
    /// or width of the longer leg for angles, or width of the back-to-back legs of long legs back-to-back double angles,
    /// or width of the outstanding legs of short legs back-to-back double angles, in. (mm)
    pub b_lower: Option<f64>,
    /// (ID) Inside diameter of pipe, in. (mm)
    pub id: Option<f64>,
    /// Thickness of web, in. (mm)
    pub tw: Option<f64>,
    ///Detailing value of web thickness, in. (mm)
    pub twdet: Option<f64>,
    /// (twdet/2) Detailing value of tw/2, in. (mm)
    pub twdet_2: Option<f64>,
    /// Thickness of flange, in. (mm)
    pub tf: Option<f64>,
    /// Detailing value of flange thickness, in. (mm)
    pub tfdet: Option<f64>,
    /// Thickness of angle leg, in. (mm)
    pub t_lower: Option<f64>,
    /// Nominal thickness of HSS and pipe wall, in. (mm)
    pub t_nom: Option<f64>,
    /// Design thickness of HSS and pipe wall, in. (mm)
    pub tdes: Option<f64>,
    /// Distance from outer face of flange to web toe of fillet used for design, in. (mm)
    pub kdes: Option<f64>,
    /// Distance from outer face of flange to web toe of fillet used for detailing, in. (mm)
    pub kdet: Option<f64>,
    /// Distance from web center line to flange toe of fillet used for detailing, in. (mm)
    pub k1: Option<f64>,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub x_lower: Option<f64>,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to center of gravity of member, in. (mm)
    pub y_lower: Option<f64>,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to shear center of member, in. (mm)
    pub eo: Option<f64>,
    /// Horizontal distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub xp: Option<f64>,
    /// Vertical distance from designated edge of member,
    /// as defined in the AISC Steel Construction Manual Part 1,
    /// to plastic neutral axis of member, in. (mm)
    pub yp: Option<f64>,
    /// (bf/2tf) Slenderness ratio for W, M, S, HP, WT, and ST flange
    pub bf_2tf: Option<f64>,
    /// (b/t) Slenderness ratio for angles and channel flange
    pub b_t: Option<f64>,
    /// (b/tdes) Slenderness ratio for square HSS or shorter wall of rectangular HSS
    pub b_tdes: Option<f64>,
    /// (h/tw) Slenderness ratio for W, M, S, HP, or channel web
    pub h_tw: Option<f64>,
    /// (h/tdes) Slenderness ratio for square HSS or longer wall of rectangular HSS
    pub h_tdes: Option<f64>,
    /// (D/t) Slenderness ratio for round HSS and pipe (D = ID), or tee shapes (D = d)
    pub d_t: Option<f64>,
    /// (Ix) Moment of inertia about the x-axis, in.4 (´106 mm4)
    pub ix: Option<f64>,
    /// (Zx) Plastic section modulus about the x-axis, in.3 (´103 mm3)
    pub zx: Option<f64>,
    /// (Sx) Elastic section modulus about the x-axis, in.3 (´103 mm3)
    pub sx: Option<f64>,
    /// Radius of gyration about the x-axis, in. (mm)
    pub rx: Option<f64>,
    /// (Iy) Moment of inertia about the y-axis, in.4 (´106 mm4)
    pub iy: Option<f64>,
    /// (Zy) Plastic section modulus about the y-axis, in.3 (´103 mm3)
    pub zy: Option<f64>,
    /// (Sy) Elastic section modulus about the y-axis, in.3 (´103 mm3)
    pub sy: Option<f64>,
    /// Radius of gyration about the y-axis (with no separation for double angles back-to-back), in. (mm)
    pub ry: Option<f64>,
    /// (Iz) Moment of inertia about the z-axis, in.4 (´106 mm4)
    pub iz: Option<f64>,
    /// Radius of gyration about the z-axis, in. (mm)
    pub rz: Option<f64>,
    /// (Sz) Elastic section modulus about the z-axis, in.3 (´103 mm3). For single angles, see SzA, SzB, and SzC.
    pub sz: Option<f64>,
    /// (J) Torsional constant, in.4 (´103 mm4)
    pub j_upper: Option<f64>,
    /// (Cw) Warping constant, in.6 (´109 mm6)
    pub cw: Option<f64>,
    /// (C) HSS torsional constant, in.3 (´103 mm3)
    pub c_upper: Option<f64>,
    /// (WNo) Normalized warping function, as used in Design Guide 9, in.2 (mm2)
    pub wno: Option<f64>,
    /// (Sw1) Warping statical moment at point 1 on cross section,
    /// as used in AISC Design Guide 9 and shown in Figures 1 and 2, in.4 (´106 mm4)
    pub sw1: Option<f64>,
    /// (Sw2) Warping statical moment at point 2 on cross section,
    /// as used in AISC Design Guide 9 and shown in Figure 2, in.4 (´106 mm4)
    pub sw2: Option<f64>,
    /// (Sw3) Warping statical moment at point 3 on cross section,
    /// as used in AISC Design Guide 9 and shown in Figure 2, in.4 (´106 mm4)
    pub sw3: Option<f64>,
    /// (Qf) Statical moment for a point in the flange directly above the vertical edge of the web,
    /// as used in AISC Design Guide 9, in.3 (´103 mm3)
    pub qf: Option<f64>,
    /// (Qw) Statical moment for a point at mid-depth of the cross section,
    /// as used in AISC Design Guide 9, in.3 (´103 mm3)
    pub qw: Option<f64>,
    /// Polar radius of gyration about the shear center, in. (mm)
    pub ro: Option<f64>,
    /// (H) Flexural constant
    pub h_upper: Option<f64>,
    /// (tan(α)) Tangent of the angle between the y-y and z-z axes for single angles,
    /// where a is shown in Figure 3
    pub tan_a: Option<f64>,
    /// (Iw) Moment of inertia about the w-axis for single angles, in.4 (´106 mm4)
    pub iw: Option<f64>,
    /// (zA) Distance from point A to center of gravity along z-axis
    pub za: Option<f64>,
    /// (zB) Distance from point B to center of gravity along z-axis
    pub zb: Option<f64>,
    /// (zC) Distance from point C to center of gravity along z-axis
    pub zc: Option<f64>,
    /// (wA) Distance from point A to center of gravity along w-axis
    pub wa: Option<f64>,
    /// (wB) Distance from point B to center of gravity along w-axis
    pub wb: Option<f64>,
    /// (wC) Distance from point C to center of gravity along w-axis
    pub wc: Option<f64>,
    /// (SwA) Elastic section modulus about the w-axis at point A on cross section
    pub swa: Option<f64>,
    /// (SwB) Elastic section modulus about the w-axis at point B on cross section
    pub swb: Option<f64>,
    /// (SwC) Elastic section modulus about the w-axis at point C on cross section
    pub swc: Option<f64>,
    /// (SzA) Elastic section modulus about the z-axis at point A on cross section
    pub sza: Option<f64>,
    /// (SzB) Elastic section modulus about the z-axis at point B on cross section
    pub szb: Option<f64>,
    /// (SzC) Elastic section modulus about the z-axis at point C on cross section
    pub szc: Option<f64>,
    /// Effective radius of gyration, in. (mm)
    pub rts: Option<f64>,
    /// Distance between the flange centroids, in. (mm)
    pub ho: Option<f64>,
    /// (PA) Shape perimeter minus one flange surface (or short leg surface for a single angle),
    /// as used in Design Guide 19, in. (mm)
    pub pa: Option<f64>,
    /// (PA2) Single angle shape perimeter minus long leg surface,
    /// as used in AISC Design Guide 19, in. (mm)
    pub pa_2: Option<f64>,
    /// (PB) Shape perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pb: Option<f64>,
    /// (PC) Box perimeter minus one flange surface, as used in Design Guide 19, in. (mm)
    pub pc: Option<f64>,
    /// (PD) Box perimeter, as used in AISC Design Guide 19, in. (mm)
    pub pd: Option<f64>,
    /// (T) Distance between web toes of fillets at top and bottom of web, in. (mm)
    pub t: Option<f64>,
    /// (WGi) The workable gage for the inner fastener holes in the flange that provides for entering and tightening clearances and edge distance and spacing requirements.
    /// The actual size, combination, and orientation of fastener components should be compared with the geometry of the cross section to ensure compatibility.
    /// See AISC Manual Part 1 for additional information, in. (mm)
    pub wgi: Option<f64>,
    /// (WGo) The bolt spacing between inner and outer fastener holes when the workable gage is compatible with four holes across the flange. See AISC Manual Part 1 for additional information, in. (mm)
    pub wgo: Option<f64>,
}

impl<'prop, 'std_nom, 'aisc_label> ShapeBuilder<'std_nom, 'aisc_label> {
    /// Creates a new instance of [ShapeBuilder] with all
    /// fields defaulted to `None`
    pub fn new() -> Self {
        ShapeBuilder {
            edi_std_nomenclature: None,
            aisc_manual_label: None,
            t_f: None,
            w_upper: None,
            a_upper: None,
            d_lower: None,
            ddet: None,
            ht: None,
            h: None,
            od: None,
            bf: None,
            bfdet: None,
            b_upper: None,
            b_lower: None,
            id: None,
            tw: None,
            twdet: None,
            twdet_2: None,
            tf: None,
            tfdet: None,
            t_lower: None,
            t_nom: None,
            tdes: None,
            kdes: None,
            kdet: None,
            k1: None,
            x_lower: None,
            y_lower: None,
            eo: None,
            xp: None,
            yp: None,
            bf_2tf: None,
            b_t: None,
            b_tdes: None,
            h_tw: None,
            h_tdes: None,
            d_t: None,
            ix: None,
            zx: None,
            sx: None,
            rx: None,
            iy: None,
            zy: None,
            sy: None,
            ry: None,
            iz: None,
            rz: None,
            sz: None,
            j_upper: None,
            cw: None,
            c_upper: None,
            wno: None,
            sw1: None,
            sw2: None,
            sw3: None,
            qf: None,
            qw: None,
            ro: None,
            h_upper: None,
            tan_a: None,
            iw: None,
            za: None,
            zb: None,
            zc: None,
            wa: None,
            wb: None,
            wc: None,
            swa: None,
            swb: None,
            swc: None,
            sza: None,
            szb: None,
            szc: None,
            rts: None,
            ho: None,
            pa: None,
            pa_2: None,
            pb: None,
            pc: None,
            pd: None,
            t: None,
            wgi: None,
            wgo: None,
        }
    }

    #[allow(dead_code)]
    /// Assigns value for EDI Std Nomenclature
    pub fn with_edi_std_nomenclature(mut self, edi_std_nom: &'std_nom str) -> Self {
        self.edi_std_nomenclature = Some(edi_std_nom);
        self
    }

    #[allow(dead_code)]
    /// Assigns value for AISC Manual Label
    pub fn with_aisc_manual_label(mut self, aisc_label: &'aisc_label str) -> Self {
        self.aisc_manual_label = Some(aisc_label);
        self
    }

    #[allow(dead_code)]
    /// Assigns value for T_F
    pub fn with_t_f(mut self, t_f: bool) -> Self {
        self.t_f = Some(t_f);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for W
    pub fn with_w_upper(mut self, w_upper: f64) -> Self {
        self.w_upper = Some(w_upper);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for A
    pub fn with_a_upper(mut self, a_upper: f64) -> Self {
        self.a_upper = Some(a_upper);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for d
    pub fn with_d_lower(mut self, d_lower: f64) -> Self {
        self.d_lower = Some(d_lower);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for ddet
    pub fn with_ddet(mut self, ddet: f64) -> Self {
        self.ddet = Some(ddet);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Ht
    pub fn with_ht(mut self, ht: f64) -> Self {
        self.ht = Some(ht);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for h
    pub fn with_h(mut self, h: f64) -> Self {
        self.h = Some(h);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for OD
    pub fn with_od(mut self, od: f64) -> Self {
        self.od = Some(od);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for bf
    pub fn with_bf(mut self, bf: f64) -> Self {
        self.bf = Some(bf);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for bfdet
    pub fn with_bfdet(mut self, bfdet: f64) -> Self {
        self.bfdet = Some(bfdet);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for B
    pub fn with_b_upper(mut self, b_upper: f64) -> Self {
        self.b_upper = Some(b_upper);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for b
    pub fn with_b_lower(mut self, b_lower: f64) -> Self {
        self.b_lower = Some(b_lower);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for ID
    pub fn with_id(mut self, id: f64) -> Self {
        self.id = Some(id);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for tw
    pub fn with_tw(mut self, tw: f64) -> Self {
        self.tw = Some(tw);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for twdet
    pub fn with_twdet(mut self, twdet: f64) -> Self {
        self.twdet = Some(twdet);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for twdet/2
    pub fn with_twdet_2(mut self, twdet_2: f64) -> Self {
        self.twdet_2 = Some(twdet_2);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for tf
    pub fn with_tf(mut self, tf: f64) -> Self {
        self.tf = Some(tf);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for tfdet
    pub fn with_tfdet(mut self, tfdet: f64) -> Self {
        self.tfdet = Some(tfdet);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for t
    pub fn with_t_lower(mut self, t_lower: f64) -> Self {
        self.t_lower = Some(t_lower);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for tnom
    pub fn with_t_nom(mut self, t_nom: f64) -> Self {
        self.t_nom = Some(t_nom);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for tdes
    pub fn with_tdes(mut self, tdes: f64) -> Self {
        self.tdes = Some(tdes);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for kdes
    pub fn with_kdes(mut self, kdes: f64) -> Self {
        self.kdes = Some(kdes);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for kdet
    pub fn with_kdet(mut self, kdet: f64) -> Self {
        self.kdet = Some(kdet);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for k1
    pub fn with_k1(mut self, k1: f64) -> Self {
        self.k1 = Some(k1);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for x
    pub fn with_x_lower(mut self, x_lower: f64) -> Self {
        self.x_lower = Some(x_lower);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for y
    pub fn with_y_lower(mut self, y_lower: f64) -> Self {
        self.y_lower = Some(y_lower);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for eo
    pub fn with_eo(mut self, eo: f64) -> Self {
        self.eo = Some(eo);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for xp
    pub fn with_xp(mut self, xp: f64) -> Self {
        self.xp = Some(xp);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for yp
    pub fn with_yp(mut self, yp: f64) -> Self {
        self.yp = Some(yp);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for bf/2tf
    pub fn with_bf_2tf(mut self, bf_2tf: f64) -> Self {
        self.bf_2tf = Some(bf_2tf);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for b/t
    pub fn with_b_t(mut self, b_t: f64) -> Self {
        self.b_t = Some(b_t);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for b/tdes
    pub fn with_b_tdes(mut self, b_tdes: f64) -> Self {
        self.b_tdes = Some(b_tdes);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for h/tw
    pub fn with_h_tw(mut self, h_tw: f64) -> Self {
        self.h_tw = Some(h_tw);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for h/tdes
    pub fn with_h_tdes(mut self, h_tdes: f64) -> Self {
        self.h_tdes = Some(h_tdes);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for D/t
    pub fn with_d_t(mut self, d_t: f64) -> Self {
        self.d_t = Some(d_t);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Ix
    pub fn with_ix(mut self, ix: f64) -> Self {
        self.ix = Some(ix);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Zx
    pub fn with_zx(mut self, zx: f64) -> Self {
        self.zx = Some(zx);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Sx
    pub fn with_sx(mut self, sx: f64) -> Self {
        self.sx = Some(sx);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for rx
    pub fn with_rx(mut self, rx: f64) -> Self {
        self.rx = Some(rx);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Iy
    pub fn with_iy(mut self, iy: f64) -> Self {
        self.iy = Some(iy);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Zy
    pub fn with_zy(mut self, zy: f64) -> Self {
        self.zy = Some(zy);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Sy
    pub fn with_sy(mut self, sy: f64) -> Self {
        self.sy = Some(sy);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for ry
    pub fn with_ry(mut self, ry: f64) -> Self {
        self.ry = Some(ry);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Iz
    pub fn with_iz(mut self, iz: f64) -> Self {
        self.iz = Some(iz);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for rz
    pub fn with_rz(mut self, rz: f64) -> Self {
        self.rz = Some(rz);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Sz
    pub fn with_sz(mut self, sz: f64) -> Self {
        self.sz = Some(sz);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for J
    pub fn with_j_upper(mut self, j_upper: f64) -> Self {
        self.j_upper = Some(j_upper);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Cw
    pub fn with_cw(mut self, cw: f64) -> Self {
        self.cw = Some(cw);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for C
    pub fn with_c_upper(mut self, c_upper: f64) -> Self {
        self.c_upper = Some(c_upper);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Wno
    pub fn with_wno(mut self, wno: f64) -> Self {
        self.wno = Some(wno);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Sw1
    pub fn with_sw1(mut self, sw1: f64) -> Self {
        self.sw1 = Some(sw1);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Sw2
    pub fn with_sw2(mut self, sw2: f64) -> Self {
        self.sw2 = Some(sw2);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Sw3
    pub fn with_sw3(mut self, sw3: f64) -> Self {
        self.sw3 = Some(sw3);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Qf
    pub fn with_qf(mut self, qf: f64) -> Self {
        self.qf = Some(qf);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Qw
    pub fn with_qw(mut self, qw: f64) -> Self {
        self.qw = Some(qw);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for ro
    pub fn with_ro(mut self, ro: f64) -> Self {
        self.ro = Some(ro);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for H
    pub fn with_h_upper(mut self, h_upper: f64) -> Self {
        self.h_upper = Some(h_upper);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for tan(α)
    pub fn with_tan_a(mut self, tan_a: f64) -> Self {
        self.tan_a = Some(tan_a);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for Iw
    pub fn with_iw(mut self, iw: f64) -> Self {
        self.iw = Some(iw);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for zA
    pub fn with_za(mut self, za: f64) -> Self {
        self.za = Some(za);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for zB
    pub fn with_zb(mut self, zb: f64) -> Self {
        self.zb = Some(zb);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for zC
    pub fn with_zc(mut self, zc: f64) -> Self {
        self.zc = Some(zc);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for wA
    pub fn with_wa(mut self, wa: f64) -> Self {
        self.wa = Some(wa);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for wB
    pub fn with_wb(mut self, wb: f64) -> Self {
        self.wb = Some(wb);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for wC
    pub fn with_wc(mut self, wc: f64) -> Self {
        self.wc = Some(wc);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for SwA
    pub fn with_swa(mut self, swa: f64) -> Self {
        self.swa = Some(swa);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for SwB
    pub fn with_swb(mut self, swb: f64) -> Self {
        self.swb = Some(swb);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for SwC
    pub fn with_swc(mut self, swc: f64) -> Self {
        self.swc = Some(swc);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for SzA
    pub fn with_sza(mut self, sza: f64) -> Self {
        self.sza = Some(sza);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for SzB
    pub fn with_szb(mut self, szb: f64) -> Self {
        self.szb = Some(szb);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for SzC
    pub fn with_szc(mut self, szc: f64) -> Self {
        self.szc = Some(szc);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for rts
    pub fn with_rts(mut self, rts: f64) -> Self {
        self.rts = Some(rts);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for ho
    pub fn with_ho(mut self, ho: f64) -> Self {
        self.ho = Some(ho);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for PA
    pub fn with_pa(mut self, pa: f64) -> Self {
        self.pa = Some(pa);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for PA2
    pub fn with_pa_2(mut self, pa_2: f64) -> Self {
        self.pa_2 = Some(pa_2);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for PB
    pub fn with_pb(mut self, pb: f64) -> Self {
        self.pb = Some(pb);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for PC
    pub fn with_pc(mut self, pc: f64) -> Self {
        self.pc = Some(pc);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for PD
    pub fn with_pd(mut self, pd: f64) -> Self {
        self.pd = Some(pd);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for T
    pub fn with_t(mut self, t: f64) -> Self {
        self.t = Some(t);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for WGi
    pub fn with_wgi(mut self, wgi: f64) -> Self {
        self.wgi = Some(wgi);
        self
    }

    #[allow(dead_code)]
    /// Assigns a value for WGo
    pub fn with_wgo(mut self, wgo: f64) -> Self {
        self.wgo = Some(wgo);
        self
    }

    #[allow(dead_code)]
    /// Attempts to build a shape with populated shape data fields,
    /// takes a type of [T: TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>]
    /// and returns either the shape that implements the `TryFrom` trait
    /// or an error explaining why the builder was unable to build that shape
    pub fn try_build<T: TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>>(
        self,
    ) -> Result<T, <T as TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>>::Error> {
        T::try_from(self)
    }
}
