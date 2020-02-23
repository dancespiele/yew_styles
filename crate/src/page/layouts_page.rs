use components::{
    AlignContent, AlignItems, Column, ColumnAlign, ColumnLayout, Container, Direction,
    JustifyContent, Row, RowAlign, RowLayout, Wrap,
};
use yew::prelude::*;

pub struct LayoutsPage;

pub struct LayoutsPageModel;

impl Component for LayoutsPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        LayoutsPage {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::row wrap=Wrap::wrap>
                {(1..5).map(|x| LayoutsPageModel.get_layouts(x)).collect::<Html>()}
            </Container>
        }
    }
}

impl LayoutsPageModel {
    fn get_layouts(self, number: i8) -> Html {
        html! {
            <Column
                layouts=vec!(ColumnLayout::cl_xl(3), ColumnLayout::cl_l(3), ColumnLayout::cl_m(6), ColumnLayout::cl_xs(12))
            >
                <h2>{number}</h2>
            </Column>
        }
    }
}
