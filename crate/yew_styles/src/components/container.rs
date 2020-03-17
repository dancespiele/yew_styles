use crate::utils::create_style;
#[cfg(any(feature = "std_web", feature = "web_sys"))]
use yew::prelude::*;

pub struct Container {
    props: Props,
}

#[derive(Clone, Copy)]
struct ContainerModel;

#[derive(Clone)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Clone)]
pub enum Wrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone)]
pub enum Mode {
    SafeMode,
    UnsafeMode,
    NoMode,
}

#[derive(Clone)]
pub enum JustifyContent {
    FlexStart(Mode),
    FlexEnd(Mode),
    Center(Mode),
    SpaceBetween(Mode),
    SpaceAround(Mode),
    SpaceEvenly(Mode),
    Start(Mode),
    End(Mode),
    Left(Mode),
    Rigth(Mode),
}

#[derive(Clone)]
pub enum AlignItems {
    Stretch(Mode),
    FlexStart(Mode),
    FlexEnd(Mode),
    Center(Mode),
    Baseline(Mode),
    FirstBaseline(Mode),
    LastBaseline(Mode),
    Start(Mode),
    End(Mode),
    SelfStart(Mode),
    SelfEnd(Mode),
}

#[derive(Clone)]
pub enum AlignContent {
    FlexStart(Mode),
    FlexEnd(Mode),
    Center(Mode),
    SpaceBetween(Mode),
    SpaceAround(Mode),
    SpaceEvenly(Mode),
    Stretch(Mode),
    Start(Mode),
    End(Mode),
    Baseline(Mode),
    FirstBaseline(Mode),
    LastBaseline(Mode),
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    pub direction: Direction,
    pub wrap: Wrap,
    #[prop_or_default]
    pub index: i16,
    #[prop_or(JustifyContent::FlexStart(Mode::NoMode))]
    pub justify_content: JustifyContent,
    #[prop_or(AlignContent::Stretch(Mode::NoMode))]
    pub align_content: AlignContent,
    #[prop_or(AlignItems::Stretch(Mode::NoMode))]
    pub align_items: AlignItems,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub name: String,
    pub children: Children,
}

impl Component for Container {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Container { props }
    }

    fn mounted(&mut self) -> ShouldRender {
        ContainerModel.init(self.props.clone());

        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=if self.props.name == ""
                {
                    format!("container container-{} {}", self.props.index, self.props.class_name)
                } else {
                    format!("container container-{}-{} {}", self.props.name, self.props.index, self.props.class_name)
                }
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl ContainerModel {
    fn init(self, props: Props) {
        self.get_flow(props.direction, props.wrap, props.index, props.name.clone());
        self.get_justify_content(props.justify_content, props.index, props.name.clone());
        self.get_align_content(props.align_content, props.index, props.name.clone());
        self.get_align_items(props.align_items, props.index, props.name);
    }

    fn get_flow(self, direction: Direction, wrap: Wrap, index: i16, name: String) {
        let direction = match direction {
            Direction::Row => format!("row"),
            Direction::RowReverse => format!("row-reverse"),
            Direction::Column => format!("column"),
            Direction::ColumnReverse => format!("column-reverse"),
        };

        let wrap = match wrap {
            Wrap::Nowrap => format!("nowrap"),
            Wrap::Wrap => format!("wrap"),
            Wrap::WrapReverse => format!("wrap-reverse"),
        };

        let value = format!("{} {}", direction, wrap);

        create_style(
            String::from("flex-flow"),
            value,
            if name == "" {
                format!("container-{}", index)
            } else {
                format!("container-{}-{}", name, index)
            },
        );
    }

    fn get_mode(self, mode: Mode) -> String {
        match mode {
            Mode::NoMode => format!(""),
            Mode::SafeMode => format!(" safe"),
            Mode::UnsafeMode => format!(" unsafe"),
        }
    }

    fn get_justify_content(self, justify_content: JustifyContent, index: i16, name: String) {
        let value = match justify_content {
            JustifyContent::FlexStart(mode) => format!("flex-start{}", self.get_mode(mode)),
            JustifyContent::FlexEnd(mode) => format!("flex-end{}", self.get_mode(mode)),
            JustifyContent::Start(mode) => format!("start{}", self.get_mode(mode)),
            JustifyContent::End(mode) => format!("end{}", self.get_mode(mode)),
            JustifyContent::Left(mode) => format!("left{}", self.get_mode(mode)),
            JustifyContent::Center(mode) => format!("center{}", self.get_mode(mode)),
            JustifyContent::Rigth(mode) => format!("right{}", self.get_mode(mode)),
            JustifyContent::SpaceAround(mode) => format!("space-around{}", self.get_mode(mode)),
            JustifyContent::SpaceBetween(mode) => format!("space-between{}", self.get_mode(mode)),
            JustifyContent::SpaceEvenly(mode) => format!("evenly{}", self.get_mode(mode)),
        };

        create_style(
            String::from("justify-content"),
            value,
            if name == "" {
                format!("container-{}", index)
            } else {
                format!("container-{}-{}", name, index)
            },
        );
    }

    fn get_align_content(self, align_content: AlignContent, index: i16, name: String) {
        let value = match align_content {
            AlignContent::Stretch(mode) => format!("stretch{}", self.get_mode(mode)),
            AlignContent::FlexStart(mode) => format!("flex-start{}", self.get_mode(mode)),
            AlignContent::FlexEnd(mode) => format!("flex-end{}", self.get_mode(mode)),
            AlignContent::Start(mode) => format!("start{}", self.get_mode(mode)),
            AlignContent::End(mode) => format!("end{}", self.get_mode(mode)),
            AlignContent::Center(mode) => format!("center{}", self.get_mode(mode)),
            AlignContent::Baseline(mode) => format!("baseline{}", self.get_mode(mode)),
            AlignContent::FirstBaseline(mode) => format!("first-baseline{}", self.get_mode(mode)),
            AlignContent::LastBaseline(mode) => format!("last-baseline{}", self.get_mode(mode)),
            AlignContent::SpaceAround(mode) => format!("space-around{}", self.get_mode(mode)),
            AlignContent::SpaceBetween(mode) => format!("space-between{}", self.get_mode(mode)),
            AlignContent::SpaceEvenly(mode) => format!("evenly{}", self.get_mode(mode)),
        };

        create_style(
            String::from("align-content"),
            value,
            if name == "" {
                format!("container-{}", index)
            } else {
                format!("container-{}-{}", name, index)
            },
        );
    }

    fn get_align_items(self, align_items: AlignItems, index: i16, name: String) {
        let value = match align_items {
            AlignItems::Stretch(mode) => format!("stretch{}", self.get_mode(mode)),
            AlignItems::Baseline(mode) => format!("baseline{}", self.get_mode(mode)),
            AlignItems::Start(mode) => format!("start{}", self.get_mode(mode)),
            AlignItems::End(mode) => format!("end{}", self.get_mode(mode)),
            AlignItems::FlexStart(mode) => format!("start{}", self.get_mode(mode)),
            AlignItems::FlexEnd(mode) => format!("flex-end{}", self.get_mode(mode)),
            AlignItems::FirstBaseline(mode) => format!("first-baseline{}", self.get_mode(mode)),
            AlignItems::LastBaseline(mode) => format!("last-baseline{}", self.get_mode(mode)),
            AlignItems::SelfStart(mode) => format!("self-start{}", self.get_mode(mode)),
            AlignItems::SelfEnd(mode) => format!("self-end{}", self.get_mode(mode)),
            AlignItems::Center(mode) => format!("center{}", self.get_mode(mode)),
        };

        create_style(
            String::from("align-items"),
            value,
            if name == "" {
                format!("container-{}", index)
            } else {
                format!("container-{}-{}", name, index)
            },
        );
    }
}
