use crate::prelude::*;

mod properties;
pub use properties::*;

pub trait WebComponent: Sized + FromProperties<Self::Properties> {
    type Properties: Properties + PartialEq + std::fmt::Debug;

    fn instance(properties: &Self::Properties) -> Signal<Self>
    where
        Self: 'static,
    {
        let input_properties = properties;
        let mut properties = use_signal(|| input_properties.clone());
        let mut component = use_signal(|| Self::from_properties(input_properties.clone()));

        if *properties.peek() != *input_properties {
            *properties.write() = input_properties.clone();
            *component.write() = Self::from_properties(input_properties.clone());
        }
        component
    }
    fn initialize(&mut self) {}

    fn id(&self) -> Option<String> {
        None
    }

    fn on_mount(&mut self) {}

    fn on_mount_with_element(&mut self, _element: web_sys::Element) {}

    fn update(&mut self, state: Self) {
        *self = state;
    }

    fn render(component: Signal<Self>) -> Element;
}

#[macro_export]
macro_rules! expose_component {
    ($component:ident as $name:ident) => {
        #[component]
        pub fn $name(props: <$component as WebComponent>::Properties) -> Element {
            let component = $component::instance(&props);
            WebComponent::render(component)
        }
    };
}
