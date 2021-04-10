use crate::Color;

#[derive(Debug)]
pub struct NamedColor {
    pub name: &'static str,
    pub list: &'static str,
    pub value: Color,
}

impl NamedColor {
    pub const fn new(name: &'static str, list: &'static str, value: u32) -> NamedColor {
        NamedColor {
            name,
            list,
            value: Color::from_hex(value),
        }
    }

    pub fn get_decorated_name(&self, exact_flag: bool) -> String {
        format!(
            "{}{}[{}]",
            if exact_flag { "" } else { "~" },
            self.name,
            self.list
        )
    }
}

pub const fn named_color(name: &'static str, list: &'static str, value: u32) -> NamedColor {
    NamedColor::new(name, list, value)
}
