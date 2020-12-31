use yew_router::prelude::*;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/cv"]
    CV,
    #[to = "/skills"]
    Skills,
    #[to = "/publications"]
    Publications,
    #[to = "/"]
    Home,
}
