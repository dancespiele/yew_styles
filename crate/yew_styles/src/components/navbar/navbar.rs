use crate::palette::{BuildPalette, Palettes};
use crate::utils::DefaultCallback;
use yew::prelude::*;

struct Navbar {
    link: ComponentLink<Self>,
    props: NavbarProps,
}

struct NavbarProps {
    pub navbar_type: Palettes,
    pub navbar_styles: String,
    pub fixed: bool,
    pub navbar: String,
    pub onsignal: Callback<()>,
}

#[derive(Clone, Properties)]
struct Props {
    pub navbar_type: Palettes,
    pub navbar_styles: String,
    pub fixed: bool,
    pub navbar: String,
    pub onsignal: DefaultCallback<Callback<()>>,
}
