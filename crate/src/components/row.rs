use yew::prelude::*;

#[derive(Clone)]
pub enum Layout {
    rw_xs(i8),
    rw_s(i8),
    rw_m(i8),
    rw_l(i8),
    rw_xl(i8),
}

#[derive(Clone)]
pub enum RowAlign {
    auto,
    flex_start,
    flex_end,
    center,
    baseline,
    stretch,
}

pub struct Row {
    link: ComponentLink<Self>,
    props: Props
}

#[derive(Clone)]
struct RowProps {
    row_align: String,
    layouts_classes: String,
}

#[derive(Clone, Copy)]
struct RowModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub layouts: Vec<Layout>,
    pub row_align: RowAlign,
    pub onsignal: DefaultCallback<Callback<()>>,
    pub children: Children,
}

pub enum Msg {
    Clicked,
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

impl Default for RowAlign {
    fn default() -> Self {
        RowAlign::auto
    }
}

impl Component for Row {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Row {
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
        let row_props = RowProps::from(self.props.clone());

        html! {
            <div
                class=format!("{} {}", row_props.row_align, row_props.layouts_classes)
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for RowProps{
    fn from(props: Props) -> Self {
        RowProps {
            layouts_classes: RowModel.get_layout_classes(props.layouts),
            row_align: RowModel.get_row_align(props.row_align),
        }
    }
}

impl RowModel {
    fn get_layout_classes(self, layouts_prop: Vec<Layout>) -> String {
        let mut layouts = layouts_prop.into_iter().map(|layout| {
            self.get_layout(layout)
        }).collect::<String>();

        layouts.truncate(layouts.len() - 1);
        
        layouts
    }

    fn get_layout(self, layout: Layout) -> String {
        match layout {
            Layout::rw_xs(size) => format!("rw-xs-{} ", size),
            Layout::rw_s(size) => format!("rw-s-{} ", size),
            Layout::rw_m(size) => format!("rw-m-{} ", size),
            Layout::rw_l(size) => format!("rw-l-{} ", size),
            Layout::rw_xl(size) => format!("rw-xl-{} ", size),
        }
    }

    fn get_row_align(self, align: RowAlign) -> String {
        match align {
            RowAlign::auto => format!("auto"),
            RowAlign::baseline => format!("baseline"),
            RowAlign::center => format!("center"),
            RowAlign::flex_start => format!("flex-start"),
            RowAlign::flex_end => format!("flex-end"),
            RowAlign::stretch => format!("stretch"),
        }
    }
}
