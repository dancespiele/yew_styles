use utils::create_style;
use yew::prelude::*;

#[derive(Clone)]
pub enum ColumnLayout {
    cl_xs(i8),
    cl_s(i8),
    cl_m(i8),
    cl_l(i8),
    cl_xl(i8),
}

#[derive(Clone)]
pub enum ColumnAlign {
    auto,
    flex_start,
    flex_end,
    center,
    baseline,
    stretch,
}

pub enum Msg {
    Clicked,
}

pub struct Column {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
struct ColumnProps {
    layouts_classes: String,
}

#[derive(Clone, Copy)]
struct ColumnModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub layouts: Vec<ColumnLayout>,
    pub column_align: ColumnAlign,
    pub onsignal: DefaultCallback<Callback<()>>,
    pub children: Children,
}

#[derive(Clone)]
pub struct DefaultCallback<T> {
    callback: T,
}

impl Default for DefaultCallback<Callback<()>> {
    fn default() -> Self {
        let callback = DefaultCallback {
            callback: Callback::noop(),
        };

        callback
    }
}

impl Default for ColumnAlign {
    fn default() -> Self {
        ColumnAlign::auto
    }
}

impl Component for Column {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Column { link, props }
    }

    fn mounted(&mut self) -> ShouldRender {
        ColumnModel.init(self.props.clone());

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.callback.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ColumnModel.init(props.clone());

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let column_props = ColumnProps::from(self.props.clone());

        html! {
            <div
                class=format!("column {}", column_props.layouts_classes)
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for ColumnProps {
    fn from(props: Props) -> Self {
        ColumnProps {
            layouts_classes: ColumnModel.get_layout_classes(props.layouts),
        }
    }
}

impl ColumnModel {
    fn init(self, props: Props) {
        self.get_column_align(props.column_align);
    }

    fn get_layout_classes(self, layouts_prop: Vec<ColumnLayout>) -> String {
        let mut layouts = layouts_prop
            .into_iter()
            .map(|layout| self.get_layout(layout))
            .collect::<String>();

        layouts.truncate(layouts.len() - 1);
        layouts
    }

    fn get_layout(self, column_layout: ColumnLayout) -> String {
        match column_layout {
            ColumnLayout::cl_xs(size) => format!("cl-xs-{} ", size),
            ColumnLayout::cl_s(size) => format!("cl-s-{} ", size),
            ColumnLayout::cl_m(size) => format!("cl-m-{} ", size),
            ColumnLayout::cl_l(size) => format!("cl-l-{} ", size),
            ColumnLayout::cl_xl(size) => format!("cl-xl-{} ", size),
        }
    }

    fn get_column_align(self, align: ColumnAlign) {
        let value = match align {
            ColumnAlign::auto => format!("auto"),
            ColumnAlign::baseline => format!("baseline"),
            ColumnAlign::center => format!("center"),
            ColumnAlign::flex_start => format!("flex-start"),
            ColumnAlign::flex_end => format!("flex-end"),
            ColumnAlign::stretch => format!("stretch"),
        };

        create_style(String::from("rowAlign"), value, String::from("column"));
    }
}
