use colorsys::{ColorTransform, Rgb};

/// Palette of styles according with the purpose
#[derive(Clone, PartialEq)]
pub enum Palette {
    Primary,
    Secondary,
    Success,
    Info,
    Link,
    Warning,
    Danger,
    Standard,
    Clean,
}

pub fn get_palette(palette: Palette) -> String {
    match palette {
        Palette::Primary => String::from("primary"),
        Palette::Secondary => String::from("secondary"),
        Palette::Info => String::from("info"),
        Palette::Link => String::from("link"),
        Palette::Success => String::from("success"),
        Palette::Warning => String::from("warning"),
        Palette::Danger => String::from("danger"),
        Palette::Standard => String::from("standard"),
        Palette::Clean => String::from("clean"),
    }
}

/// The standard sizes of the element
#[derive(Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Big,
}

/// Standars button styles
#[derive(Clone, PartialEq)]
pub enum Style {
    Regular,
    Outline,
    Light,
}

/// Position over targeted element
#[derive(Clone, PartialEq)]
pub enum Position {
    Left,
    Right,
    Above,
    Below,
}

pub fn get_size(size: Size) -> String {
    match size {
        Size::Small => String::from("small"),
        Size::Medium => String::from("medium"),
        Size::Big => String::from("big"),
    }
}

pub fn get_style(style: Style) -> String {
    match style {
        Style::Regular => String::from("regular"),
        Style::Outline => String::from("outline"),
        Style::Light => String::from("light"),
    }
}

pub fn get_iteractions(prop: &str, color: String) -> String {
    format!(
        r#"
            &:focus{{
                {}: {};
            }}
            &:hover{{
                {}: {};
            }}
            &:active{{
                {}: {};
            }}
        "#,
        prop,
        darker(&color, -10.0),
        prop,
        darker(&color, -20.0),
        prop,
        darker(&color, -30.0),
    )
}

pub fn darker(color: &str, value: f64) -> String {
    let mut rgb = Rgb::from_hex_str(color).unwrap();
    rgb.lighten(value);
    rgb.to_hex_string()
}
