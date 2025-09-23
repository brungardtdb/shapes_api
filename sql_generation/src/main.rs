use shapes::aisc_shapes::{self, *};
use std::{error::Error, fs::File};

fn main() {
    let result = parse_csv_to_sql();
    match result {
        Ok(()) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn parse_csv_to_sql() -> Result<(), Box<dyn Error>> {
    let wide_flange_shapes = get_shapes::<WideFlange>(|r| r[TYPE_INDEX].eq("W"), parse_wide_flange);
    println!("There are {} wide flange beams", &wide_flange_shapes.len());
    let misc_beam_shapes = get_shapes::<MiscBeam>(|r| r[TYPE_INDEX].eq("M"), parse_misc_beam);
    println!("There are {} misc beams", &misc_beam_shapes.len());
    let structural_beam_shapes =
        get_shapes::<StructuralBeam>(|r| r[TYPE_INDEX].eq("S"), parse_structural_beam);
    println!(
        "There are {} structural beams",
        &structural_beam_shapes.len()
    );
    let h_pile_shapes = get_shapes::<HPile>(|r| r[TYPE_INDEX].eq("HP"), parse_h_pile);
    println!("There are {} h-piles", &h_pile_shapes.len());
    let c_channels = get_shapes::<CeeChannel>(|r| r[TYPE_INDEX].eq("C"), parse_cee_channel);
    println!("There are {} cee channels", &c_channels.len());
    let angles = get_shapes::<Angle>(|r| r[TYPE_INDEX].eq("L"), parse_angles);
    println!("There are {} angles", &angles.len());
    Ok(())
}

// higher level function to extract shapes of a given type from the
// csv file and map the data to a list of the corresponding shape
fn get_shapes<T>(
    condition: fn(&csv::StringRecord) -> bool,
    parse: fn(&csv::StringRecord) -> Result<T, MissingPropertyError>,
) -> Vec<T> {
    let mut rdr = get_csv_reader();
    let records = &rdr
        .records()
        .filter(|r: &Result<csv::StringRecord, csv::Error>| r.is_ok())
        .filter(|r| condition(*&r.as_ref().unwrap()))
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    records
        .iter()
        .map(|r| parse(r).unwrap())
        .collect::<Vec<_>>()
}

// parse one shape from one csv string record
fn parse_cee_channel(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::CeeChannel, aisc_shapes::errors::MissingPropertyError> {
    let maybe_wgi = maybe_float(&record[WGI]);
    let builder = ShapeBuilder::new()
            .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
            .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
            .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
            .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
            .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
            .with_ddet(maybe_float(&record[DDET]).unwrap())
            .with_bf(maybe_float(&record[BF]).unwrap())
            .with_bfdet(maybe_float(&record[BFDET]).unwrap())
            .with_tw(maybe_float(&record[TW]).unwrap())
            .with_twdet(maybe_float(&record[TWDET]).unwrap())
            .with_twdet_2(maybe_float(&record[TWDET_2]).unwrap())
            .with_tf(maybe_float(&record[TF]).unwrap())
            .with_tfdet(maybe_float(&record[TFDET]).unwrap())
            .with_kdes(maybe_float(&record[K_DES]).unwrap())
            .with_kdet(maybe_float(&record[K_DET]).unwrap())
            .with_x_lower(maybe_float(&record[X_LOWER]).unwrap())
            .with_eo(maybe_float(&record[EO]).unwrap())
            .with_xp(maybe_float(&record[XP]).unwrap())
            .with_b_t(maybe_float(&record[B_T]).unwrap())
            .with_h_tw(maybe_float(&record[H_TW]).unwrap())
            .with_ix(maybe_float(&record[IX]).unwrap())
            .with_zx(maybe_float(&record[ZX]).unwrap())
            .with_sx(maybe_float(&record[SX]).unwrap())
            .with_rx(maybe_float(&record[RX]).unwrap())
            .with_iy(maybe_float(&record[IY]).unwrap())
            .with_zy(maybe_float(&record[ZY]).unwrap())
            .with_sy(maybe_float(&record[SY]).unwrap())
            .with_ry(maybe_float(&record[RY]).unwrap())
            .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
            .with_cw(maybe_float(&record[CW]).unwrap())
            .with_wno(maybe_float(&record[WNO]).unwrap())
            .with_sw1(maybe_float(&record[SW1]).unwrap())
            .with_sw2(maybe_float(&record[SW2]).unwrap())
            .with_sw3(maybe_float(&record[SW3]).unwrap())
            .with_qf(maybe_float(&record[QF]).unwrap())
            .with_qw(maybe_float(&record[QW]).unwrap())
            .with_ro(maybe_float(&record[RO]).unwrap())
            .with_h_upper(maybe_float(&record[H_UPPER]).unwrap())
            .with_rts(maybe_float(&record[RTS]).unwrap())
            .with_ho(maybe_float(&record[HO]).unwrap())
            .with_pa(maybe_float(&record[PA]).unwrap())
            .with_pb(maybe_float(&record[PB]).unwrap())
            .with_pc(maybe_float(&record[PC]).unwrap())
            .with_pd(maybe_float(&record[PD]).unwrap())
            .with_t(maybe_float(&record[T]).unwrap());

        if let Some(wgi) = maybe_wgi {
            return builder
            .with_wgi(wgi)
            .try_build()
        }
        builder.try_build()
    }

fn parse_h_pile(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::HPile, aisc_shapes::errors::MissingPropertyError> {
    ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
        .with_ddet(maybe_float(&record[DDET]).unwrap())
        .with_bf(maybe_float(&record[BF]).unwrap())
        .with_bfdet(maybe_float(&record[BFDET]).unwrap())
        .with_tw(maybe_float(&record[TW]).unwrap())
        .with_twdet(maybe_float(&record[TWDET]).unwrap())
        .with_twdet_2(maybe_float(&record[TWDET_2]).unwrap())
        .with_tf(maybe_float(&record[TF]).unwrap())
        .with_tfdet(maybe_float(&record[TFDET]).unwrap())
        .with_kdes(maybe_float(&record[K_DES]).unwrap())
        .with_kdet(maybe_float(&record[K_DET]).unwrap())
        .with_k1(maybe_float(&record[K1]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_h_tw(maybe_float(&record[H_TW]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_cw(maybe_float(&record[CW]).unwrap())
        .with_wno(maybe_float(&record[WNO]).unwrap())
        .with_sw1(maybe_float(&record[SW1]).unwrap())
        .with_qf(maybe_float(&record[QF]).unwrap())
        .with_qw(maybe_float(&record[QW]).unwrap())
        .with_rts(maybe_float(&record[RTS]).unwrap())
        .with_ho(maybe_float(&record[HO]).unwrap())
        .with_pa(maybe_float(&record[PA]).unwrap())
        .with_pb(maybe_float(&record[PB]).unwrap())
        .with_pc(maybe_float(&record[PC]).unwrap())
        .with_pd(maybe_float(&record[PD]).unwrap())
        .with_t(maybe_float(&record[T]).unwrap())
        .with_wgi(maybe_float(&record[WGI]).unwrap())
        .try_build()
}

fn parse_structural_beam(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::StructuralBeam, aisc_shapes::errors::MissingPropertyError> {
    let maybe_wgi = maybe_float(&record[WGI]);

    let builder = ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
        .with_ddet(maybe_float(&record[DDET]).unwrap())
        .with_bf(maybe_float(&record[BF]).unwrap())
        .with_bfdet(maybe_float(&record[BFDET]).unwrap())
        .with_tw(maybe_float(&record[TW]).unwrap())
        .with_twdet(maybe_float(&record[TWDET]).unwrap())
        .with_twdet_2(maybe_float(&record[TWDET_2]).unwrap())
        .with_tf(maybe_float(&record[TF]).unwrap())
        .with_tfdet(maybe_float(&record[TFDET]).unwrap())
        .with_kdes(maybe_float(&record[K_DES]).unwrap())
        .with_kdet(maybe_float(&record[K_DET]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_h_tw(maybe_float(&record[H_TW]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_cw(maybe_float(&record[CW]).unwrap())
        .with_wno(maybe_float(&record[WNO]).unwrap())
        .with_sw1(maybe_float(&record[SW1]).unwrap())
        .with_qf(maybe_float(&record[QF]).unwrap())
        .with_qw(maybe_float(&record[QW]).unwrap())
        .with_rts(maybe_float(&record[RTS]).unwrap())
        .with_ho(maybe_float(&record[HO]).unwrap())
        .with_pa(maybe_float(&record[PA]).unwrap())
        .with_pb(maybe_float(&record[PB]).unwrap())
        .with_pc(maybe_float(&record[PC]).unwrap())
        .with_pd(maybe_float(&record[PD]).unwrap())
        .with_t(maybe_float(&record[T]).unwrap());

    if let Some(wgi) = maybe_wgi {
        return builder.with_wgi(wgi).try_build();
    }
    builder.try_build()
}

fn parse_misc_beam(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::MiscBeam, aisc_shapes::errors::MissingPropertyError> {
    let maybe_wgi = maybe_float(&record[WGI]);
    let maybe_t_f = maybe_bool(&record[T_F]);

    let builder = ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
        .with_ddet(maybe_float(&record[DDET]).unwrap())
        .with_bf(maybe_float(&record[BF]).unwrap())
        .with_bfdet(maybe_float(&record[BFDET]).unwrap())
        .with_tw(maybe_float(&record[TW]).unwrap())
        .with_twdet(maybe_float(&record[TWDET]).unwrap())
        .with_twdet_2(maybe_float(&record[TWDET_2]).unwrap())
        .with_tf(maybe_float(&record[TF]).unwrap())
        .with_tfdet(maybe_float(&record[TFDET]).unwrap())
        .with_kdes(maybe_float(&record[K_DES]).unwrap())
        .with_kdet(maybe_float(&record[K_DET]).unwrap())
        .with_k1(maybe_float(&record[K1]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_h_tw(maybe_float(&record[H_TW]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_cw(maybe_float(&record[CW]).unwrap())
        .with_wno(maybe_float(&record[WNO]).unwrap())
        .with_sw1(maybe_float(&record[SW1]).unwrap())
        .with_qf(maybe_float(&record[QF]).unwrap())
        .with_qw(maybe_float(&record[QW]).unwrap())
        .with_rts(maybe_float(&record[RTS]).unwrap())
        .with_ho(maybe_float(&record[HO]).unwrap())
        .with_pa(maybe_float(&record[PA]).unwrap())
        .with_pb(maybe_float(&record[PB]).unwrap())
        .with_pc(maybe_float(&record[PC]).unwrap())
        .with_pd(maybe_float(&record[PD]).unwrap())
        .with_t(maybe_float(&record[T]).unwrap());

    match (&maybe_wgi.is_some(), &maybe_t_f.is_some()) {
        (true, true) => {
            return builder
                .with_wgi(maybe_wgi.unwrap())
                .with_t_f(maybe_t_f.unwrap())
                .try_build();
        }
        (true, false) => {
            return builder.with_wgi(maybe_wgi.unwrap()).try_build();
        }
        (false, true) => {
            return builder.with_t_f(maybe_t_f.unwrap()).try_build();
        }
        (false, false) => {
            return builder.try_build();
        }
    }
}

fn parse_angles(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::Angle, aisc_shapes::errors::MissingPropertyError> {
    let maybe_h_upper = maybe_float(&record[H_UPPER]);
    let maybe_swb = maybe_float(&record[SWB]);

    let builder = ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
        .with_b_lower(maybe_float(&record[B_LOWER]).unwrap())
        .with_t_lower(maybe_float(&record[T_LOWER]).unwrap())
        .with_kdes(maybe_float(&record[K_DES]).unwrap())
        .with_kdet(maybe_float(&record[K_DET]).unwrap())
        .with_x_lower(maybe_float(&record[X_LOWER]).unwrap())
        .with_y_lower(maybe_float(&record[Y_LOWER]).unwrap())
        .with_xp(maybe_float(&record[XP]).unwrap())
        .with_yp(maybe_float(&record[YP]).unwrap())
        .with_b_t(maybe_float(&record[B_T]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_iz(maybe_float(&record[IZ]).unwrap())
        .with_rz(maybe_float(&record[RZ]).unwrap())
        .with_sz(maybe_float(&record[SZ]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_cw(maybe_float(&record[CW]).unwrap())
        .with_ro(maybe_float(&record[RO]).unwrap())
        .with_tan_a(maybe_float(&record[TAN_A]).unwrap())
        .with_iw(maybe_float(&record[IW]).unwrap())
        .with_za(maybe_float(&record[ZA]).unwrap())
        .with_zb(maybe_float(&record[ZB]).unwrap())
        .with_zc(maybe_float(&record[ZC]).unwrap())
        .with_wa(maybe_float(&record[WA]).unwrap())
        .with_wb(maybe_float(&record[WB]).unwrap())
        .with_wc(maybe_float(&record[WC]).unwrap())
        .with_swa(maybe_float(&record[SWA]).unwrap())
        .with_swc(maybe_float(&record[SWC]).unwrap())
        .with_sza(maybe_float(&record[SZA]).unwrap())
        .with_szb(maybe_float(&record[SZB]).unwrap())
        .with_szc(maybe_float(&record[SZC]).unwrap())
        .with_pa(maybe_float(&record[PA]).unwrap())
        .with_pa_2(maybe_float(&record[PA2]).unwrap())
        .with_pb(maybe_float(&record[PB]).unwrap());

    match (&maybe_h_upper.is_some(), &maybe_swb.is_some()) {
        (true, true) => builder
            .with_h_upper(maybe_h_upper.unwrap())
            .with_swb(maybe_swb.unwrap())
            .try_build(),
        (true, false) => builder.with_h_upper(maybe_h_upper.unwrap()).try_build(),
        (false, true) => builder.with_swb(maybe_swb.unwrap()).try_build(),
        (false, false) => builder.try_build(),
    }
}

fn parse_wide_flange(
    record: &csv::StringRecord,
) -> Result<shapes::aisc_shapes::WideFlange, aisc_shapes::errors::MissingPropertyError> {
    let builder = shape_builder::ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_t_f(maybe_bool(&record[T_F]).unwrap())
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
        .with_ddet(maybe_float(&record[DDET]).unwrap())
        .with_bf(maybe_float(&record[BF]).unwrap())
        .with_bfdet(maybe_float(&record[BFDET]).unwrap())
        .with_tw(maybe_float(&record[TW]).unwrap())
        .with_twdet(maybe_float(&record[TWDET]).unwrap())
        .with_twdet_2(maybe_float(&record[TWDET_2]).unwrap())
        .with_tf(maybe_float(&record[TF]).unwrap())
        .with_tfdet(maybe_float(&record[TFDET]).unwrap())
        .with_kdes(maybe_float(&record[K_DES]).unwrap())
        .with_kdet(maybe_float(&record[K_DET]).unwrap())
        .with_k1(maybe_float(&record[K1]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_h_tw(maybe_float(&record[H_TW]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_cw(maybe_float(&record[CW]).unwrap())
        .with_wno(maybe_float(&record[WNO]).unwrap())
        .with_sw1(maybe_float(&record[SW1]).unwrap())
        .with_qf(maybe_float(&record[QF]).unwrap())
        .with_qw(maybe_float(&record[QW]).unwrap())
        .with_rts(maybe_float(&record[RTS]).unwrap())
        .with_ho(maybe_float(&record[HO]).unwrap())
        .with_pa(maybe_float(&record[PA]).unwrap())
        .with_pb(maybe_float(&record[PB]).unwrap())
        .with_pc(maybe_float(&record[PC]).unwrap())
        .with_pd(maybe_float(&record[PD]).unwrap())
        .with_t(maybe_float(&record[T]).unwrap())
        .with_wgi(maybe_float(&record[WGI]).unwrap());

    if let Some(num) = maybe_float(&record[WGO]) {
        return builder.with_wgo(num).try_build::<WideFlange>();
    }
    builder.try_build::<WideFlange>()
}

// helper functions
fn get_csv_reader() -> csv::Reader<File> {
    let file_result = File::open("aisc-shapes-database-v16.0.csv");
    let file = file_result.unwrap();
    csv::Reader::from_reader(file)
}

fn maybe_bool(data: &str) -> Option<bool> {
    if data.trim().eq("T") {
        return Some(true);
    } else if data.trim().eq("F") {
        return Some(false);
    }
    None
}

fn maybe_float(data: &str) -> Option<f64> {
    if *&data.trim().eq("–") {
        return None;
    } else {
        let segments: Vec<&str> = data.trim().split_whitespace().collect();
        if &segments.len() == &(0 as usize) {
            panic!("Invalid value for fraction");
        } else if &segments.len() == &(1 as usize) {
            if *&segments[0].contains("/") {
                match maybe_get_float_from_fraction("0.0", segments[0]) {
                    Some(num) => return Some(num),
                    None => {
                        dbg!(&data);
                        dbg!(&segments);
                        panic!("Could not parse fraction {}", segments[0])
                    }
                }
            } else {
                return Some(segments[0].parse::<f64>().unwrap());
            }
        } else {
            // more than one segment
            match maybe_get_float_from_fraction(segments[0], segments[1]) {
                Some(num) => return Some(num),
                None => {
                    dbg!(&data);
                    dbg!(&segments);
                    panic!("Could not parse fraction {}", segments[0])
                }
            }
        }
    }
}

fn maybe_get_float_from_fraction(whole_num: &str, fraction: &str) -> Option<f64> {
    let parsed_num: f64 = whole_num.parse().expect("Failed to parse f64");

    if fraction.trim().eq("1/16") {
        return Some(parsed_num + 0.0625);
    }
    if fraction.trim().eq("1/8") {
        return Some(parsed_num + 0.125);
    }
    if fraction.trim().eq("3/16") {
        return Some(parsed_num + 0.1875);
    }
    if fraction.trim().eq("1/4") {
        return Some(parsed_num + 0.25);
    }
    if fraction.trim().eq("5/16") {
        return Some(parsed_num + 0.3125);
    }
    if fraction.trim().eq("3/8") {
        return Some(parsed_num + 0.375);
    }
    if fraction.trim().eq("7/16") {
        return Some(parsed_num + 0.4375);
    }
    if fraction.trim().eq("1/2") {
        return Some(parsed_num + 0.5);
    }
    if fraction.trim().eq("9/16") {
        return Some(parsed_num + 0.5625);
    }
    if fraction.trim().eq("5/8") {
        return Some(parsed_num + 0.625);
    }
    if fraction.trim().eq("3/4") {
        return Some(parsed_num + 0.75);
    }
    if fraction.trim().eq("7/8") {
        return Some(parsed_num + 0.875);
    }
    if fraction.trim().eq("11/16") {
        return Some(parsed_num + 0.6875);
    }
    if fraction.trim().eq("13/16") {
        return Some(parsed_num + 0.8125);
    }
    if fraction.trim().eq("15/16") {
        return Some(parsed_num + 0.9375);
    }
    return None;
}

// indices of properties in the CSV file
static TYPE_INDEX: usize = 0;
static EDI_NOM: usize = 1;
static AISC_MAN_LBL: usize = 2;
static T_F: usize = 3;
static W_UPPER: usize = 4;
static A_UPPER: usize = 5;
static D_LOWER: usize = 6;
static DDET: usize = 7;
static HT: usize = 8;
static H: usize = 9;
static OD: usize = 10;
static BF: usize = 11;
static BFDET: usize = 12;
static B_UPPER: usize = 13;
static B_LOWER: usize = 14;
static ID: usize = 15;
static TW: usize = 16;
static TWDET: usize = 17;
static TWDET_2: usize = 18;
static TF: usize = 19;
static TFDET: usize = 20;
static T_LOWER: usize = 21;
static T_NOM: usize = 22;
static T_DES: usize = 23;
static K_DES: usize = 24;
static K_DET: usize = 25;
static K1: usize = 26;
static X_LOWER: usize = 27;
static Y_LOWER: usize = 28;
static EO: usize = 29;
static XP: usize = 30;
static YP: usize = 31;
static BF_2TF: usize = 32;
static B_T: usize = 33;
static B_TDES: usize = 34;
static H_TW: usize = 35;
static H_TDES: usize = 36;
static D_T: usize = 37;
static IX: usize = 38;
static ZX: usize = 39;
static SX: usize = 40;
static RX: usize = 41;
static IY: usize = 42;
static ZY: usize = 43;
static SY: usize = 44;
static RY: usize = 45;
static IZ: usize = 46;
static RZ: usize = 47;
static SZ: usize = 48;
static J_UPPER: usize = 49;
static CW: usize = 50;
static C_UPPER: usize = 51;
static WNO: usize = 52;
static SW1: usize = 53;
static SW2: usize = 54;
static SW3: usize = 55;
static QF: usize = 56;
static QW: usize = 57;
static RO: usize = 58;
static H_UPPER: usize = 59;
static TAN_A: usize = 60;
static IW: usize = 61;
static ZA: usize = 62;
static ZB: usize = 63;
static ZC: usize = 64;
static WA: usize = 65;
static WB: usize = 66;
static WC: usize = 67;
static SWA: usize = 68;
static SWB: usize = 69;
static SWC: usize = 70;
static SZA: usize = 71;
static SZB: usize = 72;
static SZC: usize = 73;
static RTS: usize = 74;
static HO: usize = 75;
static PA: usize = 76;
static PA2: usize = 77;
static PB: usize = 78;
static PC: usize = 79;
static PD: usize = 80;
static T: usize = 81;
static WGI: usize = 82;
static WGO: usize = 83;
