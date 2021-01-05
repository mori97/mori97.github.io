//! 折り畳み可能なコンテンツ
//! CV の職務内容で使う
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub title: String,
    pub content: Html,
}

pub enum Msg {
    Click,
}

pub struct CollapsibleContent {
    props: Props,
    link: ComponentLink<Self>,
    is_open: bool,
}

impl Component for CollapsibleContent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            is_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.is_open = !self.is_open;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick_cb = self.link.callback(|_| Msg::Click);
        if self.is_open {
            html! {
                <>
                  <div class="collapsible_title" onclick=onclick_cb>
                    <i class="fas fa-minus-square"></i>{" "}<a>{&self.props.title}</a>
                  </div>
                  <div class="collapsible_content">
                    {self.props.content.clone()}
                  </div>
                </>
            }
        } else {
            html! {
                <>
                  <div class="collapsible_title" onclick=onclick_cb>
                    <i class="fas fa-plus-square"></i>{" "}<a>{&self.props.title}</a>
                  </div>
                </>
            }
        }
    }
}
