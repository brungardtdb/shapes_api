#[derive(Debug)]
#[allow(dead_code)]
pub struct ShapeBuilder<'std_nom, 'aisc_label> {
    pub edi_std_nomenclature: Option<&'std_nom str>,
    pub aisc_manual_label: Option<&'aisc_label str>,
    pub t_f: Option<bool>,
    pub w_upper: Option<f64>,
    pub a_upper: Option<f64>,
    pub d_lower: Option<f64>,
    pub ddet: Option<f64>,
    pub ht: Option<f64>,
    pub h: Option<f64>,
    pub od: Option<f64>,
    pub bf: Option<f64>,
    pub bfdet: Option<f64>,
    pub b_upper: Option<f64>,
    pub b_lower: Option<f64>,
    pub id: Option<f64>,
    pub tw: Option<f64>,
    pub twdet: Option<f64>,
    pub twdet_2: Option<f64>,
    pub tf: Option<f64>,
    pub tfdet: Option<f64>,
    pub t_lower: Option<f64>,
    pub t_nom: Option<f64>,
    pub tdes: Option<f64>,
    pub kdes: Option<f64>,
    pub kdet: Option<f64>,
    pub k1: Option<f64>,
    pub x_lower: Option<f64>,
    pub y_lower: Option<f64>,
    pub eo: Option<f64>,
    pub xp: Option<f64>,
    pub yp: Option<f64>,
    pub bf_2tf: Option<f64>,
    pub b_t: Option<f64>,
    pub b_tdes: Option<f64>,
    pub h_tw: Option<f64>,
    pub h_tdes: Option<f64>,
    pub d_t: Option<f64>,
    pub ix: Option<f64>,
    pub zx: Option<f64>,
    pub sx: Option<f64>,
    pub rx: Option<f64>,
    pub iy: Option<f64>,
    pub zy: Option<f64>,
    pub sy: Option<f64>,
    pub ry: Option<f64>,
    pub iz: Option<f64>,
    pub rz: Option<f64>,
    pub sz: Option<f64>,
    pub j_upper: Option<f64>,
    pub cw: Option<f64>,
    pub c_upper: Option<f64>,
    pub wno: Option<f64>,
    pub sw1: Option<f64>,
    pub sw2: Option<f64>,
    pub sw3: Option<f64>,
    pub qf: Option<f64>,
    pub qw: Option<f64>,
    pub ro: Option<f64>,
    pub h_upper: Option<f64>,
    pub tan_a: Option<f64>,
    pub iw: Option<f64>,
    pub za: Option<f64>,
    pub zb: Option<f64>,
    pub zc: Option<f64>,
    pub wa: Option<f64>,
    pub wb: Option<f64>,
    pub wc: Option<f64>,
    pub swa: Option<f64>,
    pub swb: Option<f64>,
    pub swc: Option<f64>,
    pub sza: Option<f64>,
    pub szb: Option<f64>,
    pub szc: Option<f64>,
    pub rts: Option<f64>,
    pub ho: Option<f64>,
    pub pa: Option<f64>,
    pub pa_2: Option<f64>,
    pub pb: Option<f64>,
    pub pc: Option<f64>,
    pub pd: Option<f64>,
    pub t: Option<f64>,
    pub wgi: Option<f64>,
    pub wgo: Option<f64>,
}

impl<'prop, 'std_nom, 'aisc_label> ShapeBuilder<'std_nom, 'aisc_label> {
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
    pub fn with_edi_std_nomenclature(mut self, edi_std_nom: &'std_nom str) -> Self {
        self.edi_std_nomenclature = Some(edi_std_nom);
        self
    }

    #[allow(dead_code)]
    pub fn with_aisc_manual_label(mut self, aisc_label: &'aisc_label str) -> Self {
        self.aisc_manual_label = Some(aisc_label);
        self
    }

    #[allow(dead_code)]
    pub fn with_t_f(mut self, t_f: bool) -> Self {
        self.t_f = Some(t_f);
        self
    }
    #[allow(dead_code)]
    pub fn with_w_upper(mut self, w_upper: f64) -> Self {
        self.w_upper = Some(w_upper);
        self
    }

    #[allow(dead_code)]
    pub fn with_a_upper(mut self, a_upper: f64) -> Self {
        self.a_upper = Some(a_upper);
        self
    }

    #[allow(dead_code)]
    pub fn with_d_lower(mut self, d_lower: f64) -> Self {
        self.d_lower = Some(d_lower);
        self
    }

    #[allow(dead_code)]
    pub fn with_ddet(mut self, ddet: f64) -> Self {
        self.ddet = Some(ddet);
        self
    }

    #[allow(dead_code)]
    pub fn with_ht(mut self, ht: f64) -> Self {
        self.ht = Some(ht);
        self
    }

    #[allow(dead_code)]
    pub fn with_h(mut self, h: f64) -> Self {
        self.h = Some(h);
        self
    }

    #[allow(dead_code)]
    pub fn with_od(mut self, od: f64) -> Self {
        self.od = Some(od);
        self
    }

    #[allow(dead_code)]
    pub fn with_bf(mut self, bf: f64) -> Self {
        self.bf = Some(bf);
        self
    }

    #[allow(dead_code)]
    pub fn with_bfdet(mut self, bfdet: f64) -> Self {
        self.bfdet = Some(bfdet);
        self
    }

    #[allow(dead_code)]
    pub fn with_b_upper(mut self, b_upper: f64) -> Self {
        self.b_upper = Some(b_upper);
        self
    }

    #[allow(dead_code)]
    pub fn with_b_lower(mut self, b_lower: f64) -> Self {
        self.b_lower = Some(b_lower);
        self
    }

    #[allow(dead_code)]
    pub fn with_id(mut self, id: f64) -> Self {
        self.id = Some(id);
        self
    }

    #[allow(dead_code)]
    pub fn with_tw(mut self, tw: f64) -> Self {
        self.tw = Some(tw);
        self
    }

    #[allow(dead_code)]
    pub fn with_twdet(mut self, twdet: f64) -> Self {
        self.twdet = Some(twdet);
        self
    }

    #[allow(dead_code)]
    pub fn with_twdet_2(mut self, twdet_2: f64) -> Self {
        self.twdet_2 = Some(twdet_2);
        self
    }

    #[allow(dead_code)]
    pub fn with_tf(mut self, tf: f64) -> Self {
        self.tf = Some(tf);
        self
    }

    #[allow(dead_code)]
    pub fn with_tfdet(mut self, tfdet: f64) -> Self {
        self.tfdet = Some(tfdet);
        self
    }

    #[allow(dead_code)]
    pub fn with_t_lower(mut self, t_lower: f64) -> Self {
        self.t_lower = Some(t_lower);
        self
    }

    #[allow(dead_code)]
    pub fn with_t_nom(mut self, t_nom: f64) -> Self {
        self.t_nom = Some(t_nom);
        self
    }

    #[allow(dead_code)]
    pub fn with_tdes(mut self, tdes: f64) -> Self {
        self.tdes = Some(tdes);
        self
    }

    #[allow(dead_code)]
    pub fn with_kdes(mut self, kdes: f64) -> Self {
        self.kdes = Some(kdes);
        self
    }

    #[allow(dead_code)]
    pub fn with_kdet(mut self, kdet: f64) -> Self {
        self.kdet = Some(kdet);
        self
    }

    #[allow(dead_code)]
    pub fn with_k1(mut self, k1: f64) -> Self {
        self.k1 = Some(k1);
        self
    }

    #[allow(dead_code)]
    pub fn with_x_lower(mut self, x_lower: f64) -> Self {
        self.x_lower = Some(x_lower);
        self
    }

    #[allow(dead_code)]
    pub fn with_y_lower(mut self, y_lower: f64) -> Self {
        self.y_lower = Some(y_lower);
        self
    }

    #[allow(dead_code)]
    pub fn with_eo(mut self, eo: f64) -> Self {
        self.eo = Some(eo);
        self
    }

    #[allow(dead_code)]
    pub fn with_xp(mut self, xp: f64) -> Self {
        self.xp = Some(xp);
        self
    }

    #[allow(dead_code)]
    pub fn with_yp(mut self, yp: f64) -> Self {
        self.yp = Some(yp);
        self
    }

    #[allow(dead_code)]
    pub fn with_bf_2tf(mut self, bf_2tf: f64) -> Self {
        self.bf_2tf = Some(bf_2tf);
        self
    }

    #[allow(dead_code)]
    pub fn with_b_t(mut self, b_t: f64) -> Self {
        self.b_t = Some(b_t);
        self
    }

    #[allow(dead_code)]
    pub fn with_b_tdes(mut self, b_tdes: f64) -> Self {
        self.b_tdes = Some(b_tdes);
        self
    }

    #[allow(dead_code)]
    pub fn with_h_tw(mut self, h_tw: f64) -> Self {
        self.h_tw = Some(h_tw);
        self
    }

    #[allow(dead_code)]
    pub fn with_h_tdes(mut self, h_tdes: f64) -> Self {
        self.h_tdes = Some(h_tdes);
        self
    }

    #[allow(dead_code)]
    pub fn with_d_t(mut self, d_t: f64) -> Self {
        self.d_t = Some(d_t);
        self
    }

    #[allow(dead_code)]
    pub fn with_ix(mut self, ix: f64) -> Self {
        self.ix = Some(ix);
        self
    }

    #[allow(dead_code)]
    pub fn with_zx(mut self, zx: f64) -> Self {
        self.zx = Some(zx);
        self
    }

    #[allow(dead_code)]
    pub fn with_sx(mut self, sx: f64) -> Self {
        self.sx = Some(sx);
        self
    }

    #[allow(dead_code)]
    pub fn with_rx(mut self, rx: f64) -> Self {
        self.rx = Some(rx);
        self
    }

    #[allow(dead_code)]
    pub fn with_iy(mut self, iy: f64) -> Self {
        self.iy = Some(iy);
        self
    }

    #[allow(dead_code)]
    pub fn with_zy(mut self, zy: f64) -> Self {
        self.zy = Some(zy);
        self
    }

    #[allow(dead_code)]
    pub fn with_sy(mut self, sy: f64) -> Self {
        self.sy = Some(sy);
        self
    }

    #[allow(dead_code)]
    pub fn with_ry(mut self, ry: f64) -> Self {
        self.ry = Some(ry);
        self
    }

    #[allow(dead_code)]
    pub fn with_iz(mut self, iz: f64) -> Self {
        self.iz = Some(iz);
        self
    }

    #[allow(dead_code)]
    pub fn with_rz(mut self, rz: f64) -> Self {
        self.rz = Some(rz);
        self
    }

    #[allow(dead_code)]
    pub fn with_sz(mut self, sz: f64) -> Self {
        self.sz = Some(sz);
        self
    }

    #[allow(dead_code)]
    pub fn with_j_upper(mut self, j_upper: f64) -> Self {
        self.j_upper = Some(j_upper);
        self
    }

    #[allow(dead_code)]
    pub fn with_cw(mut self, cw: f64) -> Self {
        self.cw = Some(cw);
        self
    }

    #[allow(dead_code)]
    pub fn with_c_upper(mut self, c_upper: f64) -> Self {
        self.c_upper = Some(c_upper);
        self
    }

    #[allow(dead_code)]
    pub fn with_wno(mut self, wno: f64) -> Self {
        self.wno = Some(wno);
        self
    }

    #[allow(dead_code)]
    pub fn with_sw1(mut self, sw1: f64) -> Self {
        self.sw1 = Some(sw1);
        self
    }

    #[allow(dead_code)]
    pub fn with_sw2(mut self, sw2: f64) -> Self {
        self.sw2 = Some(sw2);
        self
    }

    #[allow(dead_code)]
    pub fn with_sw3(mut self, sw3: f64) -> Self {
        self.sw3 = Some(sw3);
        self
    }

    #[allow(dead_code)]
    pub fn with_qf(mut self, qf: f64) -> Self {
        self.qf = Some(qf);
        self
    }

    #[allow(dead_code)]
    pub fn with_qw(mut self, qw: f64) -> Self {
        self.qw = Some(qw);
        self
    }

    #[allow(dead_code)]
    pub fn with_ro(mut self, ro: f64) -> Self {
        self.ro = Some(ro);
        self
    }

    #[allow(dead_code)]
    pub fn with_h_upper(mut self, h_upper: f64) -> Self {
        self.h_upper = Some(h_upper);
        self
    }

    #[allow(dead_code)]
    pub fn with_tan_a(mut self, tan_a: f64) -> Self {
        self.tan_a = Some(tan_a);
        self
    }

    #[allow(dead_code)]
    pub fn with_iw(mut self, iw: f64) -> Self {
        self.iw = Some(iw);
        self
    }

    #[allow(dead_code)]
    pub fn with_za(mut self, za: f64) -> Self {
        self.za = Some(za);
        self
    }

    #[allow(dead_code)]
    pub fn with_zb(mut self, zb: f64) -> Self {
        self.zb = Some(zb);
        self
    }

    #[allow(dead_code)]
    pub fn with_zc(mut self, zc: f64) -> Self {
        self.zc = Some(zc);
        self
    }

    #[allow(dead_code)]
    pub fn with_wa(mut self, wa: f64) -> Self {
        self.wa = Some(wa);
        self
    }

    #[allow(dead_code)]
    pub fn with_wb(mut self, wb: f64) -> Self {
        self.wb = Some(wb);
        self
    }

    #[allow(dead_code)]
    pub fn with_wc(mut self, wc: f64) -> Self {
        self.wc = Some(wc);
        self
    }

    #[allow(dead_code)]
    pub fn with_swa(mut self, swa: f64) -> Self {
        self.swa = Some(swa);
        self
    }

    #[allow(dead_code)]
    pub fn with_swb(mut self, swb: f64) -> Self {
        self.swb = Some(swb);
        self
    }

    #[allow(dead_code)]
    pub fn with_swc(mut self, swc: f64) -> Self {
        self.swc = Some(swc);
        self
    }

    #[allow(dead_code)]
    pub fn with_sza(mut self, sza: f64) -> Self {
        self.sza = Some(sza);
        self
    }

    #[allow(dead_code)]
    pub fn with_szb(mut self, szb: f64) -> Self {
        self.szb = Some(szb);
        self
    }

    #[allow(dead_code)]
    pub fn with_szc(mut self, szc: f64) -> Self {
        self.szc = Some(szc);
        self
    }

    #[allow(dead_code)]
    pub fn with_rts(mut self, rts: f64) -> Self {
        self.rts = Some(rts);
        self
    }

    #[allow(dead_code)]
    pub fn with_ho(mut self, ho: f64) -> Self {
        self.ho = Some(ho);
        self
    }

    #[allow(dead_code)]
    pub fn with_pa(mut self, pa: f64) -> Self {
        self.pa = Some(pa);
        self
    }

    #[allow(dead_code)]
    pub fn with_pa_2(mut self, pa_2: f64) -> Self {
        self.pa_2 = Some(pa_2);
        self
    }

    #[allow(dead_code)]
    pub fn with_pb(mut self, pb: f64) -> Self {
        self.pb = Some(pb);
        self
    }

    #[allow(dead_code)]
    pub fn with_pc(mut self, pc: f64) -> Self {
        self.pc = Some(pc);
        self
    }

    #[allow(dead_code)]
    pub fn with_pd(mut self, pd: f64) -> Self {
        self.pd = Some(pd);
        self
    }

    #[allow(dead_code)]
    pub fn with_t(mut self, t: f64) -> Self {
        self.t = Some(t);
        self
    }

    #[allow(dead_code)]
    pub fn with_wgi(mut self, wgi: f64) -> Self {
        self.wgi = Some(wgi);
        self
    }

    #[allow(dead_code)]
    pub fn with_wgo(mut self, wgo: f64) -> Self {
        self.wgo = Some(wgo);
        self
    }

    #[allow(dead_code)]
    pub fn try_build<T: TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>>(
        self,
    ) -> Result<T, <T as TryFrom<ShapeBuilder<'std_nom, 'aisc_label>>>::Error> {
        T::try_from(self)
    }
}
