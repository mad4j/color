pub fn from_rgb(red: u8, green: u8, blue: u8) -> (u8, u8, u8, u8) {
    if red == 0 && green == 0 && blue == 0 {
        return (0, 0, 0, 100);
    } else {
        let c = 1.0 - red as f32 / 255.0;
        let m = 1.0 - green as f32 / 255.0;
        let y = 1.0 - blue as f32 / 255.0;

        let min: f32 = c.min(m).min(y);

        let c = 100.0 * ((c - min) / (1.0 - min));
        let m = 100.0 * ((m - min) / (1.0 - min));
        let y = 100.0 * ((y - min) / (1.0 - min));
        let k = 100.0 * min;

        (
            c.round() as u8,
            m.round() as u8,
            y.round() as u8,
            k.round() as u8,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn to_cmyk_test() {
        // Black
        assert_eq!((0, 0, 0, 100), from_rgb(0, 0, 0));

        // White
        assert_eq!((0, 0, 0, 0), from_rgb(255, 255, 255));

        // Red
        assert_eq!((0, 100, 100, 0), from_rgb(255, 0, 0));

        // Lime
        assert_eq!((100, 0, 100, 0), from_rgb(0, 255, 0));

        // Blue
        assert_eq!((100, 100, 0, 0), from_rgb(0, 0, 255));

        // Yellow
        assert_eq!((0, 0, 100, 0), from_rgb(255, 255, 0));

        // Cyan
        assert_eq!((100, 0, 0, 0), from_rgb(0, 255, 255));

        // Magenta
        assert_eq!((0, 100, 0, 0), from_rgb(255, 0, 255));

        /*
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
        */
    }
}
