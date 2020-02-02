use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Info,
    Link,
    Warning,
    Danger,
    Standard,
}

#[derive(Clone, PartialEq)]
pub enum ButtonStyle {
    Regular,
    Light,
    Outline,
}

#[derive(Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Big,
}

pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
}

pub struct ButtonProps {
    button_type: String,
    size: String,
    button_style: String,
    class_name: String,
    onsignal: Callback<()>,
    children: Children,
}

impl From<Props> for ButtonProps {
    fn from(props: Props) -> Self {
        ButtonProps {
            button_type: get_button_type(props.button_type),
            size: get_size(props.size),
            button_style: get_button_style(props.button_style),
            class_name: props.class_name,
            onsignal: props.onsignal,
            children: props.children,
        }
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    pub button_type: ButtonType,
    pub class_name: String,
    pub size: Size,
    pub button_style: ButtonStyle,
    #[props(required)]
    pub onsignal: Callback<()>,
    pub children: Children,
}

pub enum Msg {
    Clicked,
}

pub fn get_button_type(button_type: ButtonType) -> String {
    match button_type {
        ButtonType::Primary => String::from("primary"),
        ButtonType::Secondary => String::from("secondary"),
        ButtonType::Info => String::from("info"),
        ButtonType::Link => String::from("link"),
        ButtonType::Success => String::from("success"),
        ButtonType::Warning => String::from("warning"),
        ButtonType::Danger => String::from("danger"),
        ButtonType::Standard => String::from("standard"),
    }
}

pub fn get_size(size: Size) -> String {
    match size {
        Size::Small => String::from("small"),
        Size::Medium => String::from("medium"),
        Size::Big => String::from("big"),
    }
}

pub fn get_button_style(button_style: ButtonStyle) -> String {
    match button_style {
        ButtonStyle::Regular => String::from("regular"),
        ButtonStyle::Light => String::from("light"),
        ButtonStyle::Outline => String::from("outline"),
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Medium
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Standard
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle::Regular
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>)-> Self {
        Button {
            link,
            props: ButtonProps::from(props),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.button_type = get_button_type(props.button_type);
        self.props.class_name = props.class_name;
        self.props.size = get_size(props.size);
        self.props.button_style = get_button_style(props.button_style);
        self.props.onsignal = props.onsignal;
        self.props.children = props.children;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(|_| Msg::Clicked)
                class=format!("button {} {} {} {}",
                    self.props.button_type.clone(),
                    self.props.size.clone(),
                    self.props.button_style.clone(),
                    self.props.class_name.clone())
            > { self.props.children.render() }
            </button>
        }
    }
}