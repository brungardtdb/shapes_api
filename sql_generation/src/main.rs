use shapes::aisc_shapes::{self, *};
use std::{error::Error, fs::File, io::Write};

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
    let cee_channels = get_shapes::<CeeChannel>(|r| r[TYPE_INDEX].eq("C"), parse_cee_channel);
    println!("There are {} cee channels", &cee_channels.len());
    let misc_channels = get_shapes::<MiscChannel>(|r| r[TYPE_INDEX].eq("MC"), parse_misc_channel);
    println!("There are {} misc. channels", &misc_channels.len());
    let angles = get_shapes::<Angle>(|r| r[TYPE_INDEX].eq("L"), parse_angles);
    println!("There are {} angles", &angles.len());
    let wide_flange_tee_shapes =
        get_shapes::<WideFlangeTee>(|r| r[TYPE_INDEX].eq("WT"), parse_wide_flange_tee);
    println!(
        "There are {} wide flange tee shapes",
        &wide_flange_tee_shapes.len()
    );
    let misc_tee_shapes = get_shapes::<MiscTee>(|r| r[TYPE_INDEX].eq("MT"), parse_misc_tee);
    println!("There are {} misc. tee shapes", &misc_tee_shapes.len());
    let structural_tee_shapes =
        get_shapes::<StructuralTee>(|r| r[TYPE_INDEX].eq("ST"), parse_structural_tee);
    println!(
        "There are {} structural tee shapes",
        &structural_tee_shapes.len()
    );
    let double_angle_shapes =
        get_shapes::<DoubleAngle>(|r| r[TYPE_INDEX].eq("2L"), parse_double_angle);
    println!(
        "There are {} double angle shapes",
        &double_angle_shapes.len()
    );
    let hss_shapes = get_shapes::<HollowStructuralSection>(
        |r| {
            let shape_type = r[TYPE_INDEX].to_owned();
            let edi_nom = r[EDI_NOM].to_owned();
            return shape_type.eq("HSS")
                && edi_nom.chars().filter(|c| c.eq(&'X')).count() == (2 as usize);
        },
        parse_sq_rec_hss,
    );
    println!("There are {} HSS shapes", &hss_shapes.len());
    let hss_round_shapes = get_shapes(
        |r| {
            let shape_type = r[TYPE_INDEX].to_owned();
            let edi_nom = r[EDI_NOM].to_owned();
            return shape_type.eq("HSS")
                && edi_nom.chars().filter(|c| c.eq(&'X')).count() == (1 as usize);
        },
        parse_hss_round,
    );
    println!("There are {} HSS round shapes", &hss_round_shapes.len());
    let wide_flange_sql = sql_from_wide_flange(wide_flange_shapes);
    write_sql_to_file(String::from("wide_flanges.sql"), wide_flange_sql)?;
    let misc_beam_sql = sql_from_misc_beam(misc_beam_shapes);
    write_sql_to_file(String::from("misc_beams.sql"), misc_beam_sql)?;
    let structural_beam_sql = sql_from_structural_beam(structural_beam_shapes);
    write_sql_to_file(String::from("structural_beams.sql"), structural_beam_sql)?;
    let h_pile_sql = sql_from_h_piles(h_pile_shapes);
    write_sql_to_file(String::from("h_piles.sql"), h_pile_sql)?;
    let cee_channel_sql = sql_from_cee_channels(cee_channels);
    write_sql_to_file(String::from("cee_channels.sql"), cee_channel_sql)?;
    let misc_channel_sql = sql_from_misc_channels(misc_channels);
    write_sql_to_file(String::from("misc_channels.sql"), misc_channel_sql)?;
    let angle_sql = sql_from_angles(angles);
    write_sql_to_file(String::from("angles.sql"), angle_sql)?;
    let wide_flange_tee_sql = sql_from_wide_flange_tees(wide_flange_tee_shapes);
    write_sql_to_file(String::from("wide_flange_tees.sql"), wide_flange_tee_sql)?;
    let misc_tee_sql = sql_from_misc_tees(misc_tee_shapes);
    write_sql_to_file(String::from("misc_tees.sql"), misc_tee_sql)?;
    let structural_tee_sql = sql_from_structural_tees(structural_tee_shapes);
    write_sql_to_file(String::from("structural_tees.sql"), structural_tee_sql)?;
    let double_angle_sql = sql_from_double_angles(double_angle_shapes);
    write_sql_to_file(String::from("double_angles.sql"), double_angle_sql)?;
    let hss_sql = sql_from_hss(hss_shapes);
    write_sql_to_file(String::from("hollow_structural_sections.sql"), hss_sql)?;
    let hss_round_sql = sql_from_hss_round(hss_round_shapes);
    write_sql_to_file(
        String::from("round_hollow_structural_sections.sql"),
        hss_round_sql,
    )?;
    Ok(())
}

fn write_sql_to_file(file_name: String, sql: String) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(sql.as_bytes())?;
    Ok(())
}

fn nullable_sql_string<T: std::fmt::Display>(maybe_value: Option<T>) -> String {
    match maybe_value {
        Some(val) => String::from(format!("{}", val)),
        None => String::from("NULL"),
    }
}

// sql generation methods
fn sql_from_hss_round(shapes: Vec<RoundHollowStructuralSection>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO angles (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    od,
    t_nom,
    tdes,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    c_upper
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|h| hss_round_to_row(h))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn hss_round_to_row(shape: &RoundHollowStructuralSection) -> String {
    String::from(format!(
        "('{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.od,
        shape.t_nom,
        shape.tdes,
        shape.d_t,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.c_upper
    ))
}

fn sql_from_hss(shapes: Vec<HollowStructuralSection>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO angles (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    ht,
    h,
    b_upper,
    b_lower,
    t_nom,
    tdes,
    b_tdes,
    h_tdes,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    ro,
    j_upper,
    c_upper
    ) \nVALUES \n",
    );
    let rows = shapes.iter().map(|h| hss_to_row(h)).collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn hss_to_row(shape: &HollowStructuralSection) -> String {
    String::from(format!(
        "('{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.ht,
        shape.h,
        shape.b_upper,
        shape.b_lower,
        shape.t_nom,
        shape.tdes,
        shape.b_tdes,
        shape.h_tdes,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.c_upper
    ))
}

fn sql_from_double_angles(shapes: Vec<DoubleAngle>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO angles (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    y_lower,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    ro,
    h_upper,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|a| double_angle_to_row(a))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn double_angle_to_row(shape: &DoubleAngle) -> String {
    String::from(format!(
        "('{}','{}',{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.b_lower,
        shape.t_lower,
        shape.y_lower,
        shape.yp,
        shape.b_t,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.ro,
        shape.h_upper,
    ))
}

fn sql_from_structural_tees(shapes: Vec<StructuralTee>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO wide_flanges (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    y_lower,
    yp,
    bf_2tf,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    ro,
    h_upper,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|m| structural_tee_to_row(m))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn structural_tee_to_row(shape: &StructuralTee) -> String {
    String::from(format!(
        "('{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.y_lower,
        shape.yp,
        shape.bf_2tf,
        shape.d_t,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.ro,
        shape.h_upper,
        nullable_sql_string(shape.wgi)
    ))
}

fn sql_from_misc_tees(shapes: Vec<MiscTee>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO wide_flanges (
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    y_lower,
    yp,
    bf_2tf,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    ro,
    h_upper,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|m| misc_tee_to_row(m))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn misc_tee_to_row(shape: &MiscTee) -> String {
    String::from(format!(
        "('{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.t_f,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.y_lower,
        shape.yp,
        shape.bf_2tf,
        shape.d_t,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.ro,
        shape.h_upper,
        nullable_sql_string(shape.wgi)
    ))
}

fn sql_from_wide_flange_tees(shapes: Vec<WideFlangeTee>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO wide_flanges (
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    y_lower,
    yp,
    bf_2tf,
    d_t,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    ro,
    h_upper,
    pa,
    pb,
    pc,
    pd,
    wgi,
    wgo,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|w| wide_flange_tee_to_row(w))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn wide_flange_tee_to_row(shape: &WideFlangeTee) -> String {
    String::from(format!(
        "('{}','{}',{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.t_f,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.y_lower,
        shape.yp,
        shape.bf_2tf,
        shape.d_t,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.ro,
        shape.h_upper,
        shape.pa,
        shape.pb,
        shape.pc,
        shape.wgi,
        nullable_sql_string(shape.wgo)
    ))
}

fn sql_from_angles(shapes: Vec<Angle>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO angles (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    b_lower,
    t_lower,
    kdes,
    kdet,
    x_lower,
    y_lower,
    xp,
    yp,
    b_t,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    iz,
    rz,
    sz,
    j_upper,
    cw,
    ro,
    h_upper,
    tan_a,
    iw,
    za,
    zb,
    zc,
    wa,
    wb,
    wc,
    swa,
    swb,
    swc,
    sza,
    szb,
    szc,
    pa,
    pa_2,
    pb,
    ) \nVALUES \n",
    );
    let rows = shapes.iter().map(|c| angle_to_row(c)).collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn angle_to_row(shape: &Angle) -> String {
    String::from(format!(
        "('{}','{}',{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.b_lower,
        shape.t_lower,
        shape.kdes,
        shape.kdet,
        shape.x_lower,
        shape.y_lower,
        shape.xp,
        shape.yp,
        shape.b_t,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.iz,
        shape.rz,
        shape.sz,
        shape.j_upper,
        shape.cw,
        shape.ro,
        nullable_sql_string(shape.h_upper),
        shape.tan_a,
        shape.iw,
        shape.za,
        shape.zb,
        shape.zc,
        shape.wa,
        shape.wb,
        shape.wc,
        shape.swa,
        nullable_sql_string(shape.swb),
        shape.swc,
        shape.sza,
        shape.szb,
        shape.szc,
        shape.pa,
        shape.pa_2,
        shape.pb,
    ))
}

fn sql_from_misc_channels(shapes: Vec<MiscChannel>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO misc_channels (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    x_lower,
    eo,
    xp,
    b_t,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    sw2,
    sw3,
    qf,
    qw,
    ro,
    h_upper
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|c| misc_channel_to_row(c))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn misc_channel_to_row(shape: &MiscChannel) -> String {
    String::from(format!(
        "('{}','{}',{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.x_lower,
        shape.eo,
        shape.xp,
        shape.b_t,
        shape.h_tw,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.wno,
        shape.sw1,
        shape.sw2,
        shape.sw3,
        shape.qf,
        shape.qw,
        shape.ro,
        shape.h_upper,
        shape.rts,
        shape.ho,
        shape.pa,
        shape.pb,
        shape.pc,
        shape.pd,
        shape.t,
        nullable_sql_string(shape.wgi)
    ))
}

fn sql_from_cee_channels(shapes: Vec<CeeChannel>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO cee_channels (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    x_lower,
    eo,
    xp,
    b_t,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    sw2,
    sw3,
    qf,
    qw,
    ro,
    h_upper
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|c| cee_channel_to_row(c))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn cee_channel_to_row(shape: &CeeChannel) -> String {
    String::from(format!(
        "('{}','{}',{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.x_lower,
        shape.eo,
        shape.xp,
        shape.b_t,
        shape.h_tw,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.wno,
        shape.sw1,
        shape.sw2,
        shape.sw3,
        shape.qf,
        shape.qw,
        shape.ro,
        shape.h_upper,
        shape.rts,
        shape.ho,
        shape.pa,
        shape.pb,
        shape.pc,
        shape.pd,
        shape.t,
        nullable_sql_string(shape.wgi)
    ))
}

fn sql_from_h_piles(shapes: Vec<HPile>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO h_piles (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    k1,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes.iter().map(|h| h_pile_to_row(h)).collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn h_pile_to_row(shape: &HPile) -> String {
    String::from(format!(
        "('{}','{}',{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.k1,
        shape.bf_2tf,
        shape.h_tw,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.wno,
        shape.sw1,
        shape.qf,
        shape.qw,
        shape.rts,
        shape.ho,
        shape.pa,
        shape.pb,
        shape.pc,
        shape.pd,
        shape.t,
        shape.wgi
    ))
}

fn sql_from_structural_beam(shapes: Vec<StructuralBeam>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO structural_beams (
    edi_std_nomenclature,
    aisc_manual_label,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    k1,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|s| structural_beam_to_row(s))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn structural_beam_to_row(shape: &StructuralBeam) -> String {
    String::from(format!(
        "('{}','{}',{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        shape.edi_std_nomenclature,
        shape.aisc_manual_label,
        shape.w_upper,
        shape.a_upper,
        shape.d_lower,
        shape.ddet,
        shape.bf,
        shape.bfdet,
        shape.tw,
        shape.twdet,
        shape.twdet_2,
        shape.tf,
        shape.tfdet,
        shape.kdes,
        shape.kdet,
        shape.bf_2tf,
        shape.h_tw,
        shape.ix,
        shape.zx,
        shape.sx,
        shape.rx,
        shape.iy,
        shape.zy,
        shape.sy,
        shape.ry,
        shape.j_upper,
        shape.cw,
        shape.wno,
        shape.sw1,
        shape.qf,
        shape.qw,
        shape.rts,
        shape.ho,
        shape.pa,
        shape.pb,
        shape.pc,
        shape.pd,
        shape.t,
        nullable_sql_string(shape.wgi)
    ))
}

fn sql_from_misc_beam(shapes: Vec<MiscBeam>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO misc_beams (
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    k1,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|m| misc_beam_to_row(m))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn misc_beam_to_row(misc_beam: &MiscBeam) -> String {
    String::from(format!(
        "('{}','{}',{},{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        misc_beam.edi_std_nomenclature,
        misc_beam.aisc_manual_label,
        nullable_sql_string(misc_beam.t_f),
        misc_beam.w_upper,
        misc_beam.a_upper,
        misc_beam.d_lower,
        misc_beam.ddet,
        misc_beam.bf,
        misc_beam.bfdet,
        misc_beam.tw,
        misc_beam.twdet,
        misc_beam.twdet_2,
        misc_beam.tf,
        misc_beam.tfdet,
        misc_beam.kdes,
        misc_beam.kdet,
        misc_beam.k1,
        misc_beam.bf_2tf,
        misc_beam.h_tw,
        misc_beam.ix,
        misc_beam.zx,
        misc_beam.sx,
        misc_beam.rx,
        misc_beam.iy,
        misc_beam.zy,
        misc_beam.sy,
        misc_beam.ry,
        misc_beam.j_upper,
        misc_beam.cw,
        misc_beam.wno,
        misc_beam.sw1,
        misc_beam.qf,
        misc_beam.qw,
        misc_beam.rts,
        misc_beam.ho,
        misc_beam.pa,
        misc_beam.pb,
        misc_beam.pc,
        misc_beam.pd,
        misc_beam.t,
        nullable_sql_string(misc_beam.wgi)
    ))
}

fn sql_from_wide_flange(shapes: Vec<WideFlange>) -> String {
    let mut sql = String::new();
    sql.push_str(
        "INSERT INTO wide_flanges (
    edi_std_nomenclature,
    aisc_manual_label,
    t_f,
    w_upper,
    a_upper,
    d_lower,
    ddet,
    bf,
    bfdet,
    tw,
    twdet,
    twdet_2,
    tf,
    tfdet,
    kdes,
    kdet,
    k1,
    bf_2tf,
    h_tw,
    ix,
    zx,
    sx,
    rx,
    iy
    zy,
    sy,
    ry,
    j_upper,
    cw,
    wno,
    sw1,
    qf,
    qw,
    rts,
    ho,
    pa,
    pb,
    pc,
    pd,
    t,
    wgi,
    wgo,
    ) \nVALUES \n",
    );
    let rows = shapes
        .iter()
        .map(|w| wide_flange_to_row(w))
        .collect::<Vec<_>>();
    let row_string = rows.join(", \n");
    sql.push_str(&row_string);
    sql.push_str(";");
    sql
}

fn wide_flange_to_row(wide_flange: &WideFlange) -> String {
    String::from(format!(
        "('{}','{}',{},{},{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
        wide_flange.edi_std_nomenclature,
        wide_flange.aisc_manual_label,
        wide_flange.t_f,
        wide_flange.w_upper,
        wide_flange.a_upper,
        wide_flange.d_lower,
        wide_flange.ddet,
        wide_flange.bf,
        wide_flange.bfdet,
        wide_flange.tw,
        wide_flange.twdet,
        wide_flange.twdet_2,
        wide_flange.tf,
        wide_flange.tfdet,
        wide_flange.kdes,
        wide_flange.kdet,
        wide_flange.k1,
        wide_flange.bf_2tf,
        wide_flange.h_tw,
        wide_flange.ix,
        wide_flange.zx,
        wide_flange.sx,
        wide_flange.rx,
        wide_flange.iy,
        wide_flange.zy,
        wide_flange.sy,
        wide_flange.ry,
        wide_flange.j_upper,
        wide_flange.cw,
        wide_flange.wno,
        wide_flange.sw1,
        wide_flange.qf,
        wide_flange.qw,
        wide_flange.rts,
        wide_flange.ho,
        wide_flange.pa,
        wide_flange.pb,
        wide_flange.pc,
        wide_flange.pd,
        wide_flange.t,
        wide_flange.wgi,
        nullable_sql_string(wide_flange.wgo)
    ))
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
fn parse_hss_round(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::RoundHollowStructuralSection, aisc_shapes::errors::MissingPropertyError> {
    ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_od(maybe_float(&record[OD]).unwrap())
        .with_t_nom(maybe_float(&record[T_NOM]).unwrap())
        .with_tdes(maybe_float(&record[T_DES]).unwrap())
        .with_d_t(maybe_float(&record[D_T]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_c_upper(maybe_float(&record[C_UPPER]).unwrap())
        .try_into()
}

fn parse_sq_rec_hss(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::HollowStructuralSection, aisc_shapes::errors::MissingPropertyError> {
    ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_ht(maybe_float(&record[HT]).unwrap())
        .with_h(maybe_float(&record[H]).unwrap())
        .with_b_upper(maybe_float(&record[B_UPPER]).unwrap())
        .with_b_lower(maybe_float(&record[B_LOWER]).unwrap())
        .with_t_nom(maybe_float(&record[T_NOM]).unwrap())
        .with_tdes(maybe_float(&record[T_DES]).unwrap())
        .with_b_tdes(maybe_float(&record[B_TDES]).unwrap())
        .with_h_tdes(maybe_float(&record[H_TDES]).unwrap())
        .with_ix(maybe_float(&record[IX]).unwrap())
        .with_zx(maybe_float(&record[ZX]).unwrap())
        .with_sx(maybe_float(&record[SX]).unwrap())
        .with_rx(maybe_float(&record[RX]).unwrap())
        .with_iy(maybe_float(&record[IY]).unwrap())
        .with_zy(maybe_float(&record[ZY]).unwrap())
        .with_sy(maybe_float(&record[SY]).unwrap())
        .with_ry(maybe_float(&record[RY]).unwrap())
        .with_j_upper(maybe_float(&record[J_UPPER]).unwrap())
        .with_c_upper(maybe_float(&record[C_UPPER]).unwrap())
        .try_into()
}

fn parse_double_angle(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::DoubleAngle, aisc_shapes::errors::MissingPropertyError> {
    ShapeBuilder::new()
        .with_edi_std_nomenclature(String::from(&record[EDI_NOM]))
        .with_aisc_manual_label(String::from(&record[AISC_MAN_LBL]))
        .with_w_upper(maybe_float(&record[W_UPPER]).unwrap())
        .with_a_upper(maybe_float(&record[A_UPPER]).unwrap())
        .with_d_lower(maybe_float(&record[D_LOWER]).unwrap())
        .with_b_lower(maybe_float(&record[B_LOWER]).unwrap())
        .with_t_lower(maybe_float(&record[T_LOWER]).unwrap())
        .with_y_lower(maybe_float(&record[Y_LOWER]).unwrap())
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
        .with_ro(maybe_float(&record[RO]).unwrap())
        .with_h_upper(maybe_float(&record[H_UPPER]).unwrap())
        .try_into()
}
fn parse_structural_tee(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::StructuralTee, aisc_shapes::errors::MissingPropertyError> {
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
        .with_y_lower(maybe_float(&record[Y_LOWER]).unwrap())
        .with_yp(maybe_float(&record[YP]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_d_t(maybe_float(&record[D_T]).unwrap())
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
        .with_ro(maybe_float(&record[RO]).unwrap())
        .with_h_upper(maybe_float(&record[H_UPPER]).unwrap());

    if let Some(wgi) = maybe_wgi {
        return builder.with_wgi(wgi).try_into();
    }
    builder.try_into()
}

fn parse_misc_tee(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::MiscTee, aisc_shapes::errors::MissingPropertyError> {
    let maybe_wgi = maybe_float(&record[WGI]);

    let builder = ShapeBuilder::new()
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
        .with_y_lower(maybe_float(&record[Y_LOWER]).unwrap())
        .with_yp(maybe_float(&record[YP]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_d_t(maybe_float(&record[D_T]).unwrap())
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
        .with_ro(maybe_float(&record[RO]).unwrap())
        .with_h_upper(maybe_float(&record[H_UPPER]).unwrap());

    if let Some(wgi) = maybe_wgi {
        return builder.with_wgi(wgi).try_into();
    }
    builder.try_into()
}

fn parse_wide_flange_tee(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::WideFlangeTee, aisc_shapes::errors::MissingPropertyError> {
    let maybe_wgo = maybe_float(&record[WGO]);
    let builder = ShapeBuilder::new()
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
        .with_y_lower(maybe_float(&record[Y_LOWER]).unwrap())
        .with_yp(maybe_float(&record[YP]).unwrap())
        .with_bf_2tf(maybe_float(&record[BF_2TF]).unwrap())
        .with_d_t(maybe_float(&record[D_T]).unwrap())
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
        .with_ro(maybe_float(&record[RO]).unwrap())
        .with_h_upper(maybe_float(&record[H_UPPER]).unwrap())
        .with_pa(maybe_float(&record[PA]).unwrap())
        .with_pb(maybe_float(&record[PB]).unwrap())
        .with_pc(maybe_float(&record[PC]).unwrap())
        .with_pd(maybe_float(&record[PD]).unwrap())
        .with_wgi(maybe_float(&record[WGI]).unwrap());

    if let Some(wgo) = maybe_wgo {
        return builder.with_wgo(wgo).try_into();
    }
    builder.try_into()
}

fn parse_misc_channel(
    record: &csv::StringRecord,
) -> Result<aisc_shapes::MiscChannel, aisc_shapes::errors::MissingPropertyError> {
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
        return builder.with_wgi(wgi).try_into();
    }
    builder.try_into()
}

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
        return builder.with_wgi(wgi).try_into();
    }
    builder.try_into()
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
        .try_into()
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
        return builder.with_wgi(wgi).try_into();
    }
    builder.try_into()
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
                .try_into();
        }
        (true, false) => {
            return builder.with_wgi(maybe_wgi.unwrap()).try_into();
        }
        (false, true) => {
            return builder.with_t_f(maybe_t_f.unwrap()).try_into();
        }
        (false, false) => {
            return builder.try_into();
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
            .try_into(),
        (true, false) => builder.with_h_upper(maybe_h_upper.unwrap()).try_into(),
        (false, true) => builder.with_swb(maybe_swb.unwrap()).try_into(),
        (false, false) => builder.try_into(),
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
        return builder.with_wgo(num).try_into();
    }
    builder.try_into()
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
