use yew::prelude::*;
use yew_styles::layouts::{
    container::{AlignContent, AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{AlignSelf, Item, ItemLayout},
};

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
            <div>
                <h1>{"Layouts"}</h1>
                <h2>{"Wrap"}</h2>
                <Container direction=Direction::Row wrap=Wrap::Wrap name="wrap" index=0>
                    {(1..13).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"No wrap"}</h2>
                <Container direction=Direction::Row wrap=Wrap::Nowrap name="wrap" index=1>
                    {(1..13).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"Wrap reverse"}</h2>
                <Container direction=Direction::Row wrap=Wrap::WrapReverse name="wrap" index=2>
                    {(1..13).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"Row direction:"}</h2>
                <Container direction=Direction::Row wrap=Wrap::Wrap name="direction" index=0>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"Row reverse direction:"}</h2>
                <Container direction=Direction::RowReverse wrap=Wrap::Wrap name="direction" index=1>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"Column direction:"}</h2>
                <Container direction=Direction::Column wrap=Wrap::Wrap name="direction" index=2>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"Column reverse direction:"}</h2>
                <Container direction=Direction::ColumnReverse wrap=Wrap::Wrap name="direction" index=3>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h2>{"Combination of column and row direction"}</h2>
                <Container direction=Direction::Row wrap=Wrap::Wrap name="combination" index=0>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap name="combination" index=1>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Row wrap=Wrap::Wrap name="combination" index=2>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <h2>{"Justify Content"}</h2>
                <Container
                    direction=Direction::Row wrap=Wrap::Wrap
                    name="justify"
                    index=0 justify_content=JustifyContent::Rigth(Mode::NoMode)
                >
                    <Item
                        layouts=vec!(ItemLayout::ItXs(4))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap name="justify" index=1>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(4))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap name="justify" index=2>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content" target="_bank">{"Justify Content"}</a>
                </p>
                <h2>{"Align Content"}</h2>
                <Container
                    direction=Direction::Row
                    wrap=Wrap::Wrap name="align-content"
                    index=0 align_content=AlignContent::FlexEnd(Mode::NoMode)
                    class_name="align"
                >
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap name="align-content" index=1>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap name="align-content" index=2>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-content" target="_bank">{"Align Content"}</a>
                </p>
                <h2>{"Align Items"}</h2>
                <Container direction=Direction::Row wrap=Wrap::Wrap name="align-items" index=0>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container
                            direction=Direction::Column
                            wrap=Wrap::Wrap name="align-items"
                            index=1 align_items=AlignItems::Center(Mode::NoMode)
                        >
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container
                            direction=Direction::Column
                            wrap=Wrap::Wrap name="align-items"
                            index=2
                            align_items=AlignItems::FlexEnd(Mode::NoMode)
                        >
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-items" target="_bank">{"Align Items"}</a>
                </p>
                <h2>{"Align self"}</h2>
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                    <Item name="align" index=0 layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        <h3>{"start"}</h3>
                    </Item>
                    <Item name="align" index=1 layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::Center>
                        <h3>{"center"}</h3>
                    </Item>
                    <Item name="align" index=2 layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexEnd>
                        <h3>{"end"}</h3>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-self" target="_bank">{"Align Self"}</a>
                </p>
            </div>
        }
    }
}

impl LayoutsPageModel {
    fn get_items(self, number: i8) -> Html {
        html! {
            <Item
                layouts=vec!(ItemLayout::ItXl(3), ItemLayout::ItL(3), ItemLayout::ItM(6), ItemLayout::ItXs(12))
            >
                <h3>{number}</h3>
            </Item>
        }
    }
}
