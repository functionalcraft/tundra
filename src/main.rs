use palette::{Lch, Srgb, FromColor, Pixel};

// fn foo<T>(x: T) -> [u8; 3] {
//     Srgb::from_color(x).into_format().into_raw()
// }

fn main() {
    let base_0 = Lch::new(7.0, 10.0, 262.0);
    let base_4 = Lch::new(52.0, 14.0, 261.0);
    let base_7 = Lch::new(86.0, 11.0, 85.0);
    let base_8 = Lch::new(93.0, 5.0, 85.0);

    let red = Lch::new(50.0, 53.0, 38.0);
    let yellow_ = Lch::new(75.0, 52.0, 79.0);
    let yellow = Lch::new(87.0, 52.0, 93.0);
    let green = Lch::new(50.0, 41.0, 145.0);
    let blue = Lch::new(50.0, 37.0, 269.0);

    let base_0_raw: [u8; 3] = Srgb::from_color(base_0).into_format().into_raw();

    println!("title = \"Tundra color palette\"");
    println!("");
    println!("[winter-base]");
    println!("base0 = \"#{:02x?}{:02x?}{:02x?}\"", base_0_raw[0], base_0_raw[1], base_0_raw[2]);
}
