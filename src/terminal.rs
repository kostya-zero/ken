pub enum HighlightColor {
    Red,
    Green,
    Blue,
    Yellow,
    Dimmed,
}

impl From<&str> for HighlightColor {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "red" => HighlightColor::Red,
            "green" => HighlightColor::Green,
            "blue" => HighlightColor::Blue,
            "yellow" => HighlightColor::Yellow,
            "dimmed" => HighlightColor::Dimmed,
            _ => HighlightColor::Red,
        }
    }
}

pub fn highlight(text: &str, color: HighlightColor) -> String {
    let color_code = match color {
        HighlightColor::Red => "\x1b[31m",
        HighlightColor::Green => "\x1b[32m",
        HighlightColor::Blue => "\x1b[34m",
        HighlightColor::Yellow => "\x1b[33m",
        HighlightColor::Dimmed => "\x1b[2m",
    };
    let reset_code = "\x1b[0m";
    format!("{}{}{}", color_code, text, reset_code)
}
