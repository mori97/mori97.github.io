use yew_router::prelude::*;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/#/cv"]
    CV,
    #[to = "/#/demo/{id}"]
    Demo(usize),
    #[to = "/#/skills"]
    Skills,
    #[to = "/#/publications"]
    Publications,
    #[to = "/"]
    Home,
}
