use crate::color::Color;
use crate::namedcolor::NamedColor;

use itertools::Itertools;

pub fn short_report(c: Color) -> String {
    format!(
        "{} | {} | {} | {} | {} | {}",
        c.to_bar_string(10),
        c.to_hex_string(),
        c.to_rgb_string(),
        c.to_hsv_string(),
        c.to_cmyk_string(),
        c.find_nearest()
            .iter()
            .map(|x| x.get_decorated_name(c.to_hex() == x.value.to_hex()))
            .join(", ")
    )
}

pub fn summary_report(c: &NamedColor) -> String {
    format!(
        "| {} | {} | {}[{}]",
        c.value.to_bar_string(10),
        c.value.to_hex_string(),
        c.name,
        c.list
    )
}

pub fn full_report(c: Color) -> String {
    format!(
        "\nName: {}\nHex : #{}\nRGB : {}\nHSV : {}\nCMYK: {}\n\n{}\nNear to:\n{}",
        c.find_name(),
        c.to_hex_string(),
        c.to_rgb_string(),
        c.to_hsv_string(),
        c.to_cmyk_string(),
        c.to_square_string(10),
        c.find_nearest()
            .iter()
            .map(|&x| summary_report(x))
            .join("\n")
    )
}
