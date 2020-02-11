use yew::prelude::*;

#[derive(Clone)]
pub enum Layout {
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
    props: Props
}

#[derive(Clone)]
struct ColumnProps {
    column_align: String,
    layouts_classes: String,
}

#[derive(Clone, Copy)]
struct ColumnModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub layouts: Vec<Layout>,
    pub column_align: ColumnAlign,
    pub onsignal: DefaultCallback<Callback<()>>,
    pub children: Children,
}

#[derive(Clone)]
pub struct DefaultCallback<T> {
    callback: T
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
        Column {
            link,
            props
        }
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let column_props = ColumnProps::from(self.props.clone());

        html! {
            <div
                class=format!("{} {}", column_props.column_align, column_props.layouts_classes)
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for ColumnProps{
    fn from(props: Props) -> Self {
        ColumnProps {
            layouts_classes: ColumnModel.get_layout_classes(props.layouts),
            column_align: ColumnModel.get_column_align(props.column_align),
        }
    }
}

impl ColumnModel {
    fn get_layout_classes(self, layouts_prop: Vec<Layout>) -> String {
        let mut layouts = layouts_prop.into_iter().map(|layout| {
            self.get_layout(layout)
        }).collect::<String>();

        layouts.truncate(layouts.len() - 1);
        
        layouts
    }

    fn get_layout(self, layout: Layout) -> String {
        match layout {
            Layout::cl_xs(size) => format!("cl-xs-{} ", size),
            Layout::cl_s(size) => format!("cl-s-{} ", size),
            Layout::cl_m(size) => format!("cl-m-{} ", size),
            Layout::cl_l(size) => format!("cl-l-{} ", size),
            Layout::cl_xl(size) => format!("cl_xl-{} ", size),
        }
    }

    fn get_column_align(self, align: ColumnAlign) -> String {
        match align {
            ColumnAlign::auto => format!("auto"),
            ColumnAlign::baseline => format!("baseline"),
            ColumnAlign::center => format!("center"),
            ColumnAlign::flex_start => format!("flex-start"),
            ColumnAlign::flex_end => format!("flex-end"),
            ColumnAlign::stretch => format!("stretch"),
        }
    }
}
