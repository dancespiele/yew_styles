use yew::prelude::*;
use stdweb::js;

pub struct Container {
    link: ComponentLink<Self>,
    props: Props
}

#[derive(Clone, Copy)]
struct ContainerModel;

#[derive(Clone)]
struct ContainerProps {
    container_type: String,
    align_content: String,
    align_items: String,
}

#[derive(Clone)]
pub enum Direction {
    row,
    row_reverse,
    column,
    column_reverse,
}

#[derive(Clone)]
pub enum Wrap {
    nowrap,
    wrap,
    wrap_reverse
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
    pub direction: Direction,
    #[props(required)]
    pub wrap: Wrap,
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
        ContainerModel.init(props.clone());

        Container {
            link,
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ContainerModel.init(props.clone());
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                {self.props.children.render()}
            </div>
        }
    }
}

impl ContainerModel {
    fn init (self, props: Props) {
        self.get_flow(props.direction, props.wrap);
        self.get_justify_content(props.justify_content);
        self.get_align_content(props.align_content);
        self.get_align_items(props.align_items);
    } 

    fn get_flow(self, direction: Direction, wrap: Wrap) {
        let direction = match direction {
            Direction::row => format!("row"),
            Direction::row_reverse => format!("row-reverse"),
            Direction::column => format!("column"),
            Direction::column_reverse => format!("column-reverse"),
        };

        let wrap = match wrap {
            Wrap::nowrap => format!("nowrap"),
            Wrap::wrap => format!("wrap"),
            Wrap::wrap_reverse => format!("wrap_reverse")
        };

        let value = format!("{} {}", direction, wrap);

        self.create_style(String::from("flexFlow"), value);
    }

    fn get_mode(self, mode: Mode) -> String {
        match mode {
            Mode::no_mode => format!(""),
            Mode::safe_mode => format!(" safe"),
            Mode::unsafe_mode => format!(" unsafe"),
        }
    }

    fn get_justify_content(self, justify_content: JustifyContent) {
        let value = match justify_content {
            JustifyContent::flex_start(mode) => format!("flex-start{}", self.get_mode(mode)),
            JustifyContent::flex_end(mode) => format!("flex-end{}", self.get_mode(mode)),
            JustifyContent::start(mode) => format!("start{}", self.get_mode(mode)),
            JustifyContent::end(mode) => format!("end{}", self.get_mode(mode)),
            JustifyContent::left(mode) => format!("left{}", self.get_mode(mode)),
            JustifyContent::center(mode) => format!("center{}", self.get_mode(mode)),
            JustifyContent::rigth(mode) => format!("right{}", self.get_mode(mode)),
            JustifyContent::space_around(mode) => format!("space-around{}", self.get_mode(mode)),
            JustifyContent::space_between(mode) => format!("space-between{}", self.get_mode(mode)),
            JustifyContent::space_evenly(mode) => format!("evenly{}", self.get_mode(mode)),
        };

        self.create_style(String::from("justifyContent"), value)
    }

    fn get_align_content(self, align_content: AlignContent) {
        let value = match align_content {
            AlignContent::stretch(mode) => format!("stretch{}", self.get_mode(mode)),
            AlignContent::flex_start(mode) => format!("flex-start{}", self.get_mode(mode)),
            AlignContent::flex_end(mode) => format!("flex-end{}", self.get_mode(mode)),
            AlignContent::start(mode) => format!("start{}", self.get_mode(mode)),
            AlignContent::end(mode) => format!("end{}", self.get_mode(mode)),
            AlignContent::center(mode) => format!("center{}", self.get_mode(mode)),
            AlignContent::baseline(mode) => format!("baseline{}", self.get_mode(mode)),
            AlignContent::first_baseline(mode) => format!("first-baseline{}", self.get_mode(mode)),
            AlignContent::last_baseline(mode) => format!("last-baseline{}", self.get_mode(mode)),
            AlignContent::space_around(mode) => format!("space-around{}", self.get_mode(mode)),
            AlignContent::space_between(mode) => format!("space-between{}", self.get_mode(mode)),
            AlignContent::space_evenly(mode) => format!("evenly{}", self.get_mode(mode)),
        };

        self.create_style(String::from("alignContent"), value);
    }

    fn get_align_items(self, align_items: AlignItems) {
        let value = match align_items {
            AlignItems::stretch(mode) => format!("stretch{}", self.get_mode(mode)),
            AlignItems::baseline(mode) => format!("baseline{}", self.get_mode(mode)),
            AlignItems::start(mode) => format!("start{}", self.get_mode(mode)),
            AlignItems::end(mode) => format!("end{}", self.get_mode(mode)),
            AlignItems::flex_start(mode) => format!("start{}", self.get_mode(mode)),
            AlignItems::flex_end(mode) => format!("flex-end{}", self.get_mode(mode)),
            AlignItems::first_baseline(mode) => format!("first-baseline{}", self.get_mode(mode)),
            AlignItems::last_baseline(mode) => format!("last-baseline{}", self.get_mode(mode)),
            AlignItems::self_start(mode) => format!("self-start{}", self.get_mode(mode)),
            AlignItems::self_end(mode) => format!("self-end{}", self.get_mode(mode)),
            AlignItems::center(mode) => format!("center{}", self.get_mode(mode))
        };

        self.create_style(String::from("alignItems"), value);
    }

    fn create_style(self, style: String, value: String) {
        js! {
            const container = document.getElementsByClassName(".container")[0];
            container.style[@{style}] = @{value};
        }
    }
}

