use colorsys::{ColorAlpha, ColorTransform, Rgb};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ColorStyle {
    pub name: String,
    pub background: String,
    pub color: String,
    pub border_color: String,
}

impl From<(&str, &str, &str, &str)> for ColorStyle {
    fn from(color_style: (&str, &str, &str, &str)) -> Self {
        let (name, background, color, border_color) = color_style;

        Self {
            name: name.to_string(),
            background: background.to_string(),
            color: color.to_string(),
            border_color: border_color.to_string(),
        }
    }
}

pub fn get_styles() -> HashMap<&'static str, Vec<ColorStyle>> {
    let mut styles = HashMap::new();

    styles.insert(
        "regular",
        vec![
            ColorStyle::from(("standard", "#918d94", "#fff", "none")),
            ColorStyle::from(("primary", "#654016", "#fff", "none")),
            ColorStyle::from(("secondary", "#c77b21", "#fff", "none")),
            ColorStyle::from(("success", "#40C600", "#fff", "None")),
            ColorStyle::from(("info", "#008FD5", "#fff", "none")),
            ColorStyle::from(("link", "#034DA1", "#fff", "none")),
            ColorStyle::from(("warning", "#FFF200", "#000", "none")),
            ColorStyle::from(("danger", "#ed1c24", "#fff", "none")),
            ColorStyle::from(("clean", "#fff", "#313131", "none")),
        ],
    );

    styles.insert(
        "light",
        vec![
            ColorStyle::from(("standard", "#faf3f3", "#918d94", "none")),
            ColorStyle::from(("primary", "#e9d7c4", "#654016", "none")),
            ColorStyle::from(("secondary", "#ffd9ac", "#c77b21", "none")),
            ColorStyle::from(("success", "#b6f5c6", "#1ca53e", "None")),
            ColorStyle::from(("info", "#cedaff", "#008FD5", "none")),
            ColorStyle::from(("link", "#4fb0ff", "#034DA1", "none")),
            ColorStyle::from(("warning", "#fdffa8", "#99a034", "none")),
            ColorStyle::from(("danger", "#fdc5c5", "#ed1c24", "none")),
            ColorStyle::from(("clean", "#fff", "#313131", "none")),
        ],
    );

    styles.insert(
        "outline",
        vec![
            ColorStyle::from(("standard", "#fff", "#918d94", "#918d94")),
            ColorStyle::from(("primary", "#fff", "#654016", "#654016")),
            ColorStyle::from(("secondary", "#fff", "#c77b21", "#c77b21")),
            ColorStyle::from(("success", "#fff", "#40C600", "#40C600")),
            ColorStyle::from(("info", "#fff", "#008FD5", "#008FD5")),
            ColorStyle::from(("link", "#fff", "#034DA1", "#034DA1")),
            ColorStyle::from(("warning", "#fff", "#e6bd44", "#e6bd44")),
            ColorStyle::from(("danger", "#fff", "#ed1c24", "#ed1c24")),
            ColorStyle::from(("clean", "#fff", "#313131", "#313131")),
        ],
    );

    styles
}

pub fn darker(color: &str, value: f64) -> String {
    let mut rgb = Rgb::from_hex_str(color).unwrap();
    rgb.lighten(value);
    rgb.to_hex_string()
}

pub fn transparentize(color: &str, value: f64) -> String {
    let mut rgb = Rgb::from_hex_str(color).unwrap();
    rgb.opacify(value);
    rgb.to_css_string()
}
