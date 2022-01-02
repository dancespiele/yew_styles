use super::colors::{get_styles, ColorStyle};

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

pub fn get_palette_style(iteraction: bool) -> String {
    let styles = get_styles();
    let mut palette: String = String::from("");
    for (key, value) in styles {
        palette.push_str(&format!(
            r#"
            .{} {{
                {}
            }}
        "#,
            key,
            get_style_type(value)
        ));
    }

    palette
}

pub fn get_style_type(style: Vec<ColorStyle>) -> String {
    let mut style_type: String = String::from("");

    style.to_vec().iter_mut().for_each(|item| {
        style_type.push_str(&format!(
            r#"
                .{} {{
                    background-color: {};
                    color: {};
                    border: {}
                }}
                {}
            "#,
            item.name,
            item.background,
            item.color,
            if item.border_color != "none" {
                format!("1px solid {}", item.border_color)
            } else {
                "".to_string()
            },
            get_iteraction(&item.name, &item.background)
        ))
    });

    style_type
}

pub fn get_iteraction(style_name: &str, background: &str) -> String {
    format!(
        r#"
        {}:focus{{
            background-color: darken({}, 5%);
        }}
        {}:hover{{
            background-color: darken({}, 10%);
        }}
        {}:active{{
            background-color: darken({}, 15%);
        }}
    "#,
        style_name, background, style_name, background, style_name, background
    )
}
