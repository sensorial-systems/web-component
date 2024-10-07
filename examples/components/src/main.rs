#![allow(non_snake_case)]

use web_component::prelude::*;

#[derive(Clone, Debug, PartialEq, Props)]
struct ExampleAComponent {}

impl WebComponent for ExampleAComponent {
    type State = ();

    fn render(_component: Signal<Self>, _state: Signal<Self::State>) -> Element {
        rsx! {
            div {
                "Example A"
            }
        }
    }
}

impl From<ExampleAComponent> for () {
    fn from(_: ExampleAComponent) -> Self {
        ()
    }
}

expose_component!(ExampleAComponent as ExampleA);

#[derive(Clone, Debug, PartialEq, Props)]
struct ExampleBComponent {
    id: i32,
}

impl From<ExampleBComponent> for () {
    fn from(_: ExampleBComponent) -> Self {
        ()
    }
}

impl WebComponent for ExampleBComponent {
    type State = ();

    fn render(component: Signal<Self>, _state: Signal<Self::State>) -> Element {
        rsx! {
            div {
                "Example B: {component().id}"
            }
        }
    }
}

expose_component!(ExampleBComponent as ExampleB);

#[derive(Clone, Routable, Debug, PartialEq)]
enum Examples {
    #[route("/")]
    ExampleA {},
    #[route("/b/:id")]
    ExampleB { id: i32 },
}


fn main() {
    launch(App);
}


fn App() -> Element {
    rsx! {
        Router::<Examples> {}
    }
}
