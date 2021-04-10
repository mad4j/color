pub fn from_rgb(red: u8, green: u8, blue: u8) -> (u16, u8, u8) {
    let r = red as f32 / 255.0;
    let g = green as f32 / 255.0;
    let b = blue as f32 / 255.0;

    let cmax = r.max(g.max(b));
    let cmin = r.min(g.min(b));

    let diff = cmax - cmin;

    let h = if cmax == cmin {
        0.0
    } else if cmax == r {
        (60.0 * ((g - b) / diff) + 360.0) % 360.0
    } else if cmax == g {
        (60.0 * ((b - r) / diff) + 120.0) % 360.0
    } else if cmax == b {
        (60.0 * ((r - g) / diff) + 240.0) % 360.0
    } else {
        -1.0
    };

    let s = if cmax == 0.0 {
        0.0
    } else {
        (diff / cmax) * 100.0
    };

    let v = cmax * 100.0;

    (h.round() as u16, s.round() as u8, v.round() as u8)
}

#[cfg(test)]
mod tests {

    use crate::color::Color;

    #[test]
    fn to_hsv_test() {
        // Black
        assert_eq!((0, 0, 0), Color::new(0, 0, 0).to_hsv());
        // White
        assert_eq!((0, 0, 100), Color::new(255, 255, 255).to_hsv());
        // Red
        assert_eq!((0, 100, 100), Color::new(255, 0, 0).to_hsv());
        // Lime
        assert_eq!((120, 100, 100), Color::new(0, 255, 0).to_hsv());
        // Blue
        assert_eq!((240, 100, 100), Color::new(0, 0, 255).to_hsv());
        // Yellow
        assert_eq!((60, 100, 100), Color::new(255, 255, 0).to_hsv());
        // Cyan
        assert_eq!((180, 100, 100), Color::new(0, 255, 255).to_hsv());
        // Magenta
        assert_eq!((300, 100, 100), Color::new(255, 0, 255).to_hsv());
        // Silver
        assert_eq!((0, 0, 75), Color::new(191, 191, 191).to_hsv());
        // Gray
        assert_eq!((0, 0, 50), Color::new(128, 128, 128).to_hsv());
        // Maroon
        assert_eq!((0, 100, 50), Color::new(128, 0, 0).to_hsv());
        // Olive
        assert_eq!((60, 100, 50), Color::new(128, 128, 0).to_hsv());
        // Green
        assert_eq!((120, 100, 50), Color::new(0, 128, 0).to_hsv());
        // Purple
        assert_eq!((300, 100, 50), Color::new(128, 0, 128).to_hsv());
        // Teal
        assert_eq!((180, 100, 50), Color::new(0, 128, 128).to_hsv());
        // Navy
        assert_eq!((240, 100, 50), Color::new(0, 0, 128).to_hsv());
    }
}
