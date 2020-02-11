use yew::prelude::*;

pub struct Container {
    link: ComponentLink<Self>,
    props: Props
}

#[derive(Clone, Copy)]
struct ContainerModel;

#[derive(Clone)]
struct ContainerProps {
    container_type: String,
    justify_content: String,
    align_content: String,
    align_items: String,
}

#[derive(Clone)]
pub enum ContainerType {
    row,
    row_reverse,
    column,
    column_reverse,
    no_wrap,
    wrap,
    wrap_reverse,
}

#[derive(Clone)]
pub enum Mode {
    safe_mode,
    unsafe_mode,
    no_mode,
} 

#[derive(Clone)]
pub enum JustifyContent {
    flex_start(Mode),
    flex_end(Mode),
    center(Mode),
    space_between(Mode),
    space_around(Mode),
    space_evenly(Mode),
    start(Mode),
    end(Mode),
    left(Mode),
    rigth(Mode),
}

#[derive(Clone)]
pub enum AlignItems {
    stretch(Mode),
    flex_start(Mode),
    flex_end(Mode),
    center(Mode),
    baseline(Mode),
    first_baseline(Mode),
    last_baseline(Mode),
    start(Mode),
    end(Mode),
    self_start(Mode),
    self_end(Mode),
}

#[derive(Clone)]
pub enum AlignContent {
    flex_start(Mode),
    flex_end(Mode),
    center(Mode),
    space_between(Mode),
    space_around(Mode),
    space_evenly(Mode),
    stretch(Mode),
    start(Mode),
    end(Mode),
    baseline(Mode),
    first_baseline(Mode),
    last_baseline(Mode),
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub container_type: ContainerType,
    pub justify_content: JustifyContent,
    pub align_content: AlignContent,
    pub align_items: AlignItems,
    pub children: Children,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::flex_start(Mode::no_mode)
    }
}

impl Default for AlignContent {
    fn default() -> Self {
        AlignContent::stretch(Mode::no_mode)
    }
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::stretch(Mode::no_mode)
    }
}

impl Component for Container {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Container {
            link,
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let container_props = ContainerProps::from(self.props.clone());

        html! {
            <div
                class=format!(
                    "{} {} {} {}",
                    container_props.container_type,
                    container_props.justify_content,
                    container_props.align_content,
                    container_props.align_items
                )
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for ContainerProps{
    fn from(props: Props) -> Self {
        ContainerProps {
            container_type: ContainerModel.get_container_type(props.container_type),
            justify_content: ContainerModel.get_justify_content(props.justify_content),
            align_content: ContainerModel.get_align_content(props.align_content),
            align_items: ContainerModel.get_align_items(props.align_items),
        }
    }
}

impl ContainerModel {
    fn get_container_type(self, container_type: ContainerType) -> String {
        match container_type {
            ContainerType::row => format!("row"),
            ContainerType::row_reverse => format!("row-reverse"),
            ContainerType::column => format!("column"),
            ContainerType::column_reverse => format!("column-reverse"),
            ContainerType::no_wrap => format!("no-wrap"),
            ContainerType::wrap => format!("wrap"),
            ContainerType::wrap_reverse => format!("wrap-reverse"),
        }
    }

    fn get_mode(self, mode: Mode) -> String {
        match mode {
            Mode::no_mode => format!(""),
            Mode::safe_mode => format!("-safe"),
            Mode::unsafe_mode => format!("-unsafe"),
        }
    }

    fn get_justify_content(self, justify_content: JustifyContent) -> String {
        match justify_content {
            JustifyContent::flex_start(mode) => format!("justify-content-flex-start{}", self.get_mode(mode)),
            JustifyContent::flex_end(mode) => format!("justify-content-flex-end{}", self.get_mode(mode)),
            JustifyContent::start(mode) => format!("justify-content-start{}", self.get_mode(mode)),
            JustifyContent::end(mode) => format!("justify-content-end{}", self.get_mode(mode)),
            JustifyContent::left(mode) => format!("justify-content-left{}", self.get_mode(mode)),
            JustifyContent::center(mode) => format!("justify-content-center{}", self.get_mode(mode)),
            JustifyContent::rigth(mode) => format!("justify-content-right{}", self.get_mode(mode)),
            JustifyContent::space_around(mode) => format!("justify-content-space-around{}", self.get_mode(mode)),
            JustifyContent::space_between(mode) => format!("justify-content-space-between{}", self.get_mode(mode)),
            JustifyContent::space_evenly(mode) => format!("justify-content-evenly{}", self.get_mode(mode)),
        }
    }

    fn get_align_content(self, align_content: AlignContent) -> String {
        match align_content {
            AlignContent::stretch(mode) => format!("align-content-stretch{}", self.get_mode(mode)),
            AlignContent::flex_start(mode) => format!("align-content-flex-start{}", self.get_mode(mode)),
            AlignContent::flex_end(mode) => format!("align-content-flex-end{}", self.get_mode(mode)),
            AlignContent::start(mode) => format!("align-content-start{}", self.get_mode(mode)),
            AlignContent::end(mode) => format!("align-content-end{}", self.get_mode(mode)),
            AlignContent::center(mode) => format!("align-content-center{}", self.get_mode(mode)),
            AlignContent::baseline(mode) => format!("align-content-baseline{}", self.get_mode(mode)),
            AlignContent::first_baseline(mode) => format!("align-content-first-baseline{}", self.get_mode(mode)),
            AlignContent::last_baseline(mode) => format!("align-content-last-baseline{}", self.get_mode(mode)),
            AlignContent::space_around(mode) => format!("align-content-space-around{}", self.get_mode(mode)),
            AlignContent::space_between(mode) => format!("align-content-space-between{}", self.get_mode(mode)),
            AlignContent::space_evenly(mode) => format!("align-content-evenly{}", self.get_mode(mode)),
        }
    }

    fn get_align_items(self, align_items: AlignItems) -> String {
        match align_items {
            AlignItems::stretch(mode) => format!("align-items-stretch{}", self.get_mode(mode)),
            AlignItems::baseline(mode) => format!("align-items-baseline{}", self.get_mode(mode)),
            AlignItems::start(mode) => format!("align-items-start{}", self.get_mode(mode)),
            AlignItems::end(mode) => format!("align-items-end{}", self.get_mode(mode)),
            AlignItems::flex_start(mode) => format!("align-items-start{}", self.get_mode(mode)),
            AlignItems::flex_end(mode) => format!("align-items-flex-end{}", self.get_mode(mode)),
            AlignItems::first_baseline(mode) => format!("align-items-first-baseline{}", self.get_mode(mode)),
            AlignItems::last_baseline(mode) => format!("align-items-last-baseline{}", self.get_mode(mode)),
            AlignItems::self_start(mode) => format!("align-items-self-start{}", self.get_mode(mode)),
            AlignItems::self_end(mode) => format!("align-items-self-end{}", self.get_mode(mode)),
            AlignItems::center(mode) => format!("align-items-center{}", self.get_mode(mode))
        }
    }
}

