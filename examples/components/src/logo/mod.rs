use web_component::prelude::*;
use crate::navigation_bar::NavigationBar;

#[derive(Default)]
pub struct LogoComponent {}

impl WebComponent for LogoComponent {
    type Properties = NoProperties;

    fn render(_component: Signal<Self>) -> Element {
        rsx! {
            style { { include_str!("style.css") } }
            NavigationBar {}
            div { class: "container",
                div { class: "top" }
                div { class: "middle" }
                div { class: "bottom" }
            }
        }
    }
}

expose_component!(LogoComponent as Logo);
