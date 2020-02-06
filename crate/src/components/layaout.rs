use yew::prelude::*;

pub struct Layout {
    link: ComponentLink<Self>,
    props: Props
}

#[derive(Clone, Properties)]
pub struct Props {
    layaout_type: String,
    cl_xs: String,
    cl_s: String,
    cl_m: String,
    cl_l: String,
    cl_xl: String,
    rw_xs: String,
    rw_s: String,
    rw_m: String,
    rw_l: String,
    rw_xl: String,
    pub onsignal: Callback<()>,
    pub children: Children,
}

impl Default for Callback<()> {
    fn default() -> Self {
        Callback::noop()
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
                self.props.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {self.props.children.render()}
            </div>
        }
    }
}