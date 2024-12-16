use web_component::prelude::*;
use crate::navigation_bar::NavigationBar;

pub struct ParameterizedRouteComponent {
    properties: ParameterizedRouteComponentProperties,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct ParameterizedRouteComponentProperties {
    id: i32,
}

impl FromProperties<ParameterizedRouteComponentProperties> for ParameterizedRouteComponent {
    fn from_properties(properties: ParameterizedRouteComponentProperties) -> Self {
        ParameterizedRouteComponent { properties }
    }
}

impl WebComponent for ParameterizedRouteComponent {
    type Properties = ParameterizedRouteComponentProperties;

    fn render(component: Signal<Self>) -> Element {
        rsx! {
            NavigationBar {}
            div {
                "Parameterized Route: {component.read().properties.id}"
            }
        }
    }
}

expose_component!(ParameterizedRouteComponent as ParameterizedRoute);
