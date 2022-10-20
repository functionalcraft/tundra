use palette::{FromColor, Gradient, Lab, Lch, Pixel, Saturate, Shade, Srgb};
use std::error::Error;
use std::f64::consts::{FRAC_PI_2, PI};
use std::io::Write;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle};
use svg::Document;

fn hex_string(color: &Lch) -> String {
    let bytes: [u8; 3] = Srgb::from_color(color.clone()).into_format().into_raw();

    format!("#{:02x?}{:02x?}{:02x?}", bytes[0], bytes[1], bytes[2])
}

fn svg_donut_sector(
    origin: (f64, f64),
    outer_radius: f64,
    inner_radius: f64,
    n_sectors: usize,
    k_sector: usize,
    fill: &Lch,
) -> Path {
    let sector_radians: f64 = 2.0 / (n_sectors as f64) * PI;
    let start_radians = sector_radians * (0.5 - (k_sector as f64)) + FRAC_PI_2;
    let end_radians = start_radians - sector_radians;
    let outer_arc_start_x = origin.0 + outer_radius * f64::cos(start_radians);
    let outer_arc_start_y = origin.1 - outer_radius * f64::sin(start_radians);
    let outer_arc_end_x = origin.0 + outer_radius * f64::cos(end_radians);
    let outer_arc_end_y = origin.1 - outer_radius * f64::sin(end_radians);
    let inner_arc_start_x = origin.0 + inner_radius * f64::cos(end_radians);
    let inner_arc_start_y = origin.1 - inner_radius * f64::sin(end_radians);
    let inner_arc_end_x = origin.0 + inner_radius * f64::cos(start_radians);
    let inner_arc_end_y = origin.1 - inner_radius * f64::sin(start_radians);

    let data = Data::new()
        .move_to((outer_arc_start_x, outer_arc_start_y))
        .elliptical_arc_to((
            outer_radius,
            outer_radius,
            0,
            0,
            1,
            outer_arc_end_x,
            outer_arc_end_y,
        ))
        .line_to((inner_arc_start_x, inner_arc_start_y))
        .elliptical_arc_to((
            inner_radius,
            inner_radius,
            0,
            0,
            0,
            inner_arc_end_x,
            inner_arc_end_y,
        ))
        .line_to((outer_arc_start_x, outer_arc_start_y))
        .close();

    Path::new()
        .set("fill", hex_string(fill))
        .set("stroke", "none")
        .set("stroke-width", 0)
        .set("d", data)
}

fn svg_palette(
    base_gradient: &Vec<Lch>,
    background: usize,
    colors_1: &Vec<Lch>,
    colors_2: &Vec<Lch>,
    colors_3: &Vec<Lch>,
) -> Document {
    let n_sectors = colors_1.len();
    let empty_document = Document::new().set("viewBox", (0, 0, 150, 120)).set(
        "style",
        format!("background:{}", hex_string(&base_gradient[background])),
    );

    let h = 100.0 / 9.0;
    let document_with_gradient =
        base_gradient
            .into_iter()
            .enumerate()
            .fold(empty_document, |d, (k, c)| {
                d.add(
                    Rectangle::new()
                        .set("x", 10.0)
                        .set("y", 10.0 + (k as f64) * h)
                        .set("width", 20.0)
                        .set("height", h)
                        .set("fill", hex_string(c))
                        .set("stroke", "none")
                        .set("stroke-width", 0),
                )
            });

    let document_with_dark_accents = colors_1
        .into_iter()
        .enumerate()
        .fold(document_with_gradient, |d, (k, c)| {
            d.add(svg_donut_sector((90.0, 60.0), 50.0, 40.0, n_sectors, k, c))
        });

    let document_with_medium_accents = colors_2
        .into_iter()
        .enumerate()
        .fold(document_with_dark_accents, |d, (k, c)| {
            d.add(svg_donut_sector((90.0, 60.0), 37.0, 27.0, n_sectors, k, c))
        });

    colors_3
        .into_iter()
        .enumerate()
        .fold(document_with_medium_accents, |d, (k, c)| {
            d.add(svg_donut_sector((90.0, 60.0), 24.0, 14.0, n_sectors, k, c))
        })
}

fn toml_document(
    base_gradient: &Vec<Lch>,
    accents: &Vec<Lch>,
    light_accents: &Vec<Lch>,
    dark_accents: &Vec<Lch>,
    accent_names: &[&str; 12],
) -> String {
    let mut s = String::from("title = \"Tundra color palette\"\n");

    s.push_str("\n[base]\n");
    for (k, color) in base_gradient.into_iter().enumerate() {
        s.push_str(&format!("base-{} = \"{}\"\n", k, hex_string(&color)));
    }

    s.push_str("\n[accents]\n");
    for (name, color) in accent_names.into_iter().zip(accents) {
        s.push_str(&format!("{} = \"{}\"\n", name, hex_string(&color)));
    }

    for (name, color) in accent_names.into_iter().zip(light_accents) {
        s.push_str(&format!("light-{} = \"{}\"\n", name, hex_string(&color)));
    }

    for (name, color) in accent_names.into_iter().zip(dark_accents) {
        s.push_str(&format!("dark-{} = \"{}\"\n", name, hex_string(&color)));
    }

    s
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let base_0 = Lch::new(7.097, 10.318, 261.561);
    let base_4 = Lch::new(51.941, 13.891, 260.896);
    let base_7 = Lch::new(85.752, 10.91, 84.825);
    let base_8 = Lch::new(92.93, 5.094, 84.625);

    let red = Lch::new(50.0, 53.0, 38.0);
    let yellow = Lch::new(88.0, 58.0, 93.0);
    let green = Lch::new(53.0, 44.0, 145.0);
    let blue = Lch::new(50.0, 37.0, 269.0);

    let base_gradient: Vec<Lch> = Gradient::from([
        (0.0, Lab::from_color(base_0)),
        (0.5, Lab::from_color(base_4)),
        (0.875, Lab::from_color(base_7)),
        (1.0, Lab::from_color(base_8)),
    ])
    .take(9)
    .map(|c| Lch::from_color(c))
    .collect();

    let accent_names: [&str; 12] = [
        "red",
        "vermilion",
        "orange",
        "amber",
        "yellow",
        "chartreuse",
        "green",
        "teal",
        "blue",
        "violet",
        "purple",
        "magenta",
    ];
    let accents: Vec<Lch> = Gradient::from([
        (0.0, red),
        (4.0 / 12.0, yellow),
        (6.0 / 12.0, green),
        (8.0 / 12.0, blue),
        (1.0, red),
    ])
    .take(13)
    .take(12)
    .collect();

    let light_accents: Vec<Lch> = accents
        .clone()
        .into_iter()
        .map(|c| c.lighten(0.18).lighten_fixed(0.09))
        .collect();

    let dark_accents: Vec<Lch> = accents
        .clone()
        .into_iter()
        .map(|c| c.darken(0.18).darken_fixed(0.09).desaturate(0.15))
        .collect();

    svg::save(
        "palette.svg",
        &svg_palette(&base_gradient, 1, &dark_accents, &accents, &light_accents),
    )
    .unwrap();
    svg::save(
        "palette_light.svg",
        &svg_palette(&base_gradient, 7, &dark_accents, &accents, &light_accents),
    )
    .unwrap();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("Tundra.toml")
        .unwrap();
    write!(
        file,
        "{}",
        toml_document(
            &base_gradient,
            &accents,
            &light_accents,
            &dark_accents,
            &accent_names
        )
    )
    .unwrap();

    Ok(())
}
