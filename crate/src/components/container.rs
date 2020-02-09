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

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub container_type: ContainerType,
    pub children: Children,
}

pub enum Msg {}

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
        let Container_props = ContainerProps::from(self.props.clone());

        html! {
            <div
                class=format!("{}", Container_props.container_type)
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
        }
    }
}

impl ContainerModel {
    fn get_container_type(self, layaout_type: ContainerType) -> String {
        match layaout_type {
            ContainerType::row => format!("row"),
            ContainerType::row_reverse => format!("row-reverse"),
            ContainerType::column => format!("column"),
            ContainerType::column_reverse => format!("column-reverse"),
            ContainerType::no_wrap => format!("no-wrap"),
            ContainerType::wrap => format!("wrap"),
            ContainerType::wrap_reverse => format!("wrap-reverse"),
        }
    }
}

