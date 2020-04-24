/// Palette of styles according with the purpose
#[derive(Clone)]
pub enum Palette {
    Primary,
    Secondary,
    Success,
    Info,
    Link,
    Warning,
    Danger,
    Standard,
}

pub fn get_pallete(palette: Palette) -> String {
    match palette {
        Palette::Primary => String::from("primary"),
        Palette::Secondary => String::from("secondary"),
        Palette::Info => String::from("info"),
        Palette::Link => String::from("link"),
        Palette::Success => String::from("success"),
        Palette::Warning => String::from("warning"),
        Palette::Danger => String::from("danger"),
        Palette::Standard => String::from("standard"),
    }
}

/// Standars button styles
#[derive(Clone)]
pub enum Style {
    Regular,
    Outline,
    Light,
}

pub fn get_style(style: Style) -> String {
    match style {
        Style::Regular => String::from("regular"),
        Style::Outline => String::from("outline"),
        Style::Light => String::from("light"),
    }
}
