#![allow(non_snake_case)]

use web_component::prelude::*;

#[derive(Default)]
struct ExampleAComponent {}

impl WebComponent for ExampleAComponent {
    type Properties = NoProperties;

    fn render(_component: Signal<Self>) -> Element {
        rsx! {
            div {
                "Example A"
            }
        }
    }
}

expose_component!(ExampleAComponent as ExampleA);

struct ExampleBComponent {
    properties: ExampleBComponentProperties,
}

#[derive(Clone, Debug, PartialEq, Props)]
struct ExampleBComponentProperties {
    id: i32,
}

impl FromProperties<ExampleBComponentProperties> for ExampleBComponent {
    fn from_properties(properties: ExampleBComponentProperties) -> Self {
        ExampleBComponent { properties }
    }
}

impl WebComponent for ExampleBComponent {
    type Properties = ExampleBComponentProperties;

    fn render(component: Signal<Self>) -> Element {
        rsx! {
            div {
                "Example B: {component.read().properties.id}"
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
