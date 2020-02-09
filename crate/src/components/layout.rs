use yew::prelude::*;

pub struct Layout {
    link: ComponentLink<Self>,
    props: Props
}

#[derive(Clone, Copy)]
struct LayoutModel;

#[derive(Clone)]
struct LayoutProps {
    layaout_type: String,
    column_row_classes: String,
}

#[derive(Clone)]
pub struct DefaultCallback<T> {
    callback: T
}

#[derive(Clone)]
pub enum Column {
    cl_xs(i8),
    cl_s(i8),
    cl_m(i8),
    cl_l(i8),
    cl_xl(i8),
}

#[derive(Clone)]
pub enum Row {
    rw_xs(i8),
    rw_s(i8),
    rw_m(i8),
    rw_l(i8),
    rw_xl(i8),
}

#[derive(Clone)]
pub enum LayoutType {
    Container,
    Item,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub layaout_type: LayoutType,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
    pub onsignal: DefaultCallback<Callback<()>>,
    pub children: Children,
}

impl Default for DefaultCallback<Callback<()>> {
    fn default() -> Self {
        let callback = DefaultCallback {
            callback: Callback::noop(),
        };

        callback
    }
}

pub enum Msg {
    Clicked,
}

impl Component for Layout {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Layout {
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
        let layout_props = LayoutProps::from(self.props.clone());

        html! {
            <div
                class=format!("{} {}", layout_props.layaout_type, layout_props.column_row_classes)
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for LayoutProps{
    fn from(props: Props) -> Self {
        LayoutProps {
            column_row_classes: LayoutModel.get_column_row_classes(props.columns, props.rows),
            layaout_type: LayoutModel.get_layout_type(props.layaout_type),
        }
    }
}

impl LayoutModel {
    fn get_column_row_classes(self, columns_prop: Vec<Column>, rows_prop: Vec<Row>) -> String {
        let columns = columns_prop.into_iter().map(|column| {
            self.get_column(column)
        }).collect::<String>();

        let rows = rows_prop.into_iter().map(|row| {
            self.get_row(row)
        }).collect::<String>();

        let mut column_row_classes = [columns, rows].concat();
        column_row_classes.truncate(column_row_classes.len() - 1);

        column_row_classes
    }

    fn get_column(self, column: Column) -> String {
        match column {
            Column::cl_xs(size) => format!("cl-xs-{} ", size),
            Column::cl_s(size) => format!("cl-s-{} ", size),
            Column::cl_m(size) => format!("cl-m-{} ", size),
            Column::cl_l(size) => format!("cl-l-{} ", size),
            Column::cl_xl(size) => format!("cl_xl-{} ", size),
        }
    }

    fn get_row(self, row: Row) -> String {
        match row {
            Row::rw_xs(size) => format!("rw-xs-{} ", size),
            Row::rw_s(size) => format!("rw-s-{} ", size),
            Row::rw_m(size) => format!("rw-m-{} ", size),
            Row::rw_l(size) => format!("rw-l-{} ", size),
            Row::rw_xl(size) => format!("rw-xl-{} ", size),
        }
    }

    fn get_layout_type(self, layaout_type: LayoutType) -> String {
        match layaout_type {
            LayoutType::Container => format!("container"),
            LayoutType::Item => format!("item"),
        }
    }
}

