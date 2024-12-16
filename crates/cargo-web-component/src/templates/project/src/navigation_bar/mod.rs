use web_component::prelude::*;

#[derive(Default)]
pub struct NavigationBarComponent {}

impl WebComponent for NavigationBarComponent {
    type Properties = NoProperties;

    fn render(_component: Signal<Self>) -> Element {
        rsx! {
            style { { include_str!("style.css") } }
            div {
                class: "navigation",
                a { href: "/", "Logo" }
                a { href: "/b/1", "Parameterized Route" }
            }
        }
    }
}

expose_component!(NavigationBarComponent as NavigationBar);
