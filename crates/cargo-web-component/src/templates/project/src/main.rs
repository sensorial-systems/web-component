#![allow(non_snake_case)]

use web_component::prelude::*;

mod navigation_bar;
mod logo;
mod parameterized_route;

use logo::Logo;
use parameterized_route::ParameterizedRoute;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Examples {
    #[route("/")]
    Logo {},
    #[route("/b/:id")]
    ParameterizedRoute { id: i32 },
}

fn App() -> Element {
    rsx! {
        Router::<Examples> {}
    }
}

fn main() {
    launch(App);
}
