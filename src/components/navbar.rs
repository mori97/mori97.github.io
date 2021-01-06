//! ルータ機能を提供するナビゲーションバー
use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navbar {}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <ul class="navbar">
              <li class="navbar_title">
                {" Du Yicheng's HP "}
                <a class="github-icon" href="https://github.com/mori97/mori97.github.io">
                  <i class="fab fa-github-square fa-lg"></i>
                </a>
              </li>
              <li class="navbar_item"><Anchor route=AppRoute::Home>{"Home"}</Anchor></li>
              <li class="navbar_item"><Anchor route=AppRoute::CV>{"CV"}</Anchor></li>
              <li class="navbar_item"><Anchor route=AppRoute::Skills>{"Skills"}</Anchor></li>
              <li class="navbar_item"><Anchor route=AppRoute::Publications>{"Publications"}</Anchor></li>
              <li class="navbar_item dropdown">
                <a class="dropbtn">{"Demos"}</a>
                <div class="dropdown-content">
                  <a><Anchor route=AppRoute::Demo(1)>{"U-Netで音楽パート分離"}</Anchor></a>
                </div>
              </li>
              <li class="navbar_item"><a href="https://github.com/mori97">{"GitHub"}</a></li>
            </ul>
        }
    }
}
