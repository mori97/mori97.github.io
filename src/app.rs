use crate::components::Navbar;
use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(move |switch: AppRoute| match switch {
            _ => html! {
                <div>{"Under construction"}</div>
            },
        });
        html! {
            <>
              <Navbar />
              <Router<AppRoute, ()> render=render />
            </>
        }
    }
}
