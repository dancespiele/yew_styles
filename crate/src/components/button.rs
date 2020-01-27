use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Standard,
}

#[derive(Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Big,
}

pub struct Button {
    link: ComponentLink<Self>,
    button_type: String,
    class_name: String,
    size: String,
    onsignal: Callback<()>,
    children: Children,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub button_type: ButtonType,
    pub class_name: String,
    pub size: Size,
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

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>)-> Self {
        Button {
            link,
            button_type: get_button_type(props.button_type),
            class_name: props.class_name,
            size: get_size(props.size),
            onsignal: props.onsignal,
            children: props.children,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.button_type = get_button_type(props.button_type);
        self.class_name = props.class_name;
        self.size = get_size(props.size);
        self.onsignal = props.onsignal;
        self.children = props.children;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(|_| Msg::Clicked)
                class=format!("button {} {} {}", self.button_type.clone(), self.size.clone(), self.class_name.clone())
            > { self.children.render() }
            </button>
        }
    }
}