#[derive(Clone)]
pub enum Palettes {
    Primary,
    Secondary,
    Success,
    Info,
    Link,
    Warning,
    Danger,
    Standard,
}

impl Default for Palettes {
    fn default() -> Self {
        Palettes::Standard
    }
}

pub struct BuildPalette;

impl BuildPalette {
    pub fn new(palette: Palettes) -> String {
        match palette {
            Palettes::Primary => String::from("primary"),
            Palettes::Secondary => String::from("secondary"),
            Palettes::Info => String::from("info"),
            Palettes::Link => String::from("link"),
            Palettes::Success => String::from("success"),
            Palettes::Warning => String::from("warning"),
            Palettes::Danger => String::from("danger"),
            Palettes::Standard => String::from("standard"),
        }
    }
}
