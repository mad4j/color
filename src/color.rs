use crate::cmyk_model;
use crate::hsv_model;
use crate::namedcolor::NamedColor;

use crate::color_names::COLOR_NAMES;

use itertools::Itertools;

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub const fn from_hex(hex: u32) -> Color {
        let red = ((hex & 0x00FF0000) >> 16) as u8;
        let green = ((hex & 0x0000FF00) >> 8) as u8;
        let blue = (hex & 0x000000FF) as u8;

        Color { red, green, blue }
    }

    pub fn random() -> Color {
        let red: u8 = rand::random::<u8>();
        let green: u8 = rand::random::<u8>();
        let blue: u8 = rand::random::<u8>();

        Color { red, green, blue }
    }

    pub fn distance_from_hex(&self, hex: u32) -> u64 {
        let r: i64 = ((0x00FF0000 & hex) >> 16).into();
        let g: i64 = ((0x0000FF00 & hex) >> 8 as i64).into();
        let b: i64 = (0x000000FF & hex).into();

        (((self.red as i64) - r).pow(2)
            + (self.green as i64 - g).pow(2)
            + (self.blue as i64 - b).pow(2)) as u64
    }

    pub fn distance_from_color(&self, c: &Color) -> u64 {
        (((self.red as i64) - c.red as i64).pow(2)
            + (self.green as i64 - c.green as i64).pow(2)
            + (self.blue as i64 - c.blue as i64).pow(2)) as u64
    }

    pub fn find_nearest(&self) -> Vec<&NamedColor> {
        let m = COLOR_NAMES
            .iter()
            .map(|x| self.distance_from_color(&x.value))
            .min()
            .unwrap();

        COLOR_NAMES
            .iter()
            .filter(|x| self.distance_from_color(&x.value) == m)
            .collect()
    }

    pub fn find_name(&self) -> String {
        COLOR_NAMES
            .iter()
            .filter(|x| self.distance_from_color(&x.value) == 0)
            .map(|x| x.get_decorated_name(true))
            .join(", ")
    }

    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    pub fn to_cmyk(&self) -> (u8, u8, u8, u8) {
        cmyk_model::from_rgb(self.red, self.green, self.blue)
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    pub fn to_hsv(&self) -> (u16, u8, u8) {
        hsv_model::from_rgb(self.red, self.green, self.blue)
    }

    pub fn to_bar_string(&self, length: u8) -> String {
        format!(
            "\x1b[48;2;{};{};{}m{}\x1b[0m",
            self.red,
            self.green,
            self.blue,
            (0..length).map(|_| " ").collect::<String>()
        )
    }

    pub fn to_square_string(&self, length: u8) -> String {
        let l = format!(" {}\n", self.to_bar_string(length));
        (0..length / 2).map(|_| l.clone()).collect::<String>()
    }

    pub fn to_hex_string(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    pub fn to_rgb_string(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("R:{:3.0}  G:{:3.0}  B:{:3.0}", r, g, b)
    }

    pub fn to_cmyk_string(&self) -> String {
        let (c, m, y, k) = self.to_cmyk();
        format!("C:{:3.0}% M:{:3.0}% Y:{:3.0}% K:{:3.0}%", c, m, y, k)
    }

    pub fn to_hsv_string(&self) -> String {
        let (h, s, v) = self.to_hsv();
        format!("H:{:3.0}° S:{:3.0}% V:{:3.0}%", h, s, v,)
    }
}
