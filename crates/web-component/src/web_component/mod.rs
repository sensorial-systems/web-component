use crate::prelude::*;

mod properties;
pub use properties::*;

pub trait WebComponent: Sized + FromProperties<Self::Properties> {
    type Properties: Properties + PartialEq + std::fmt::Debug;

    fn instance(properties: &Self::Properties) -> Signal<Self> {
        // let mut component = use_signal(|| {
        //     let mut state = state.clone();
        //     state.initialize();
        //     state
        // });
        // use_effect(move || {
        //     component.write().on_mount();
        //     let id = component.write().id().clone();
        //     if let Some(id) = id {
        //         if let Some(window) = web_sys::window() {
        //             if let Some(document) = window.document() {
        //                 if let Some(element) = document.get_element_by_id(&id) {
        //                     component.write().on_mount_with_element(element);
        //                 }
        //             }
        //         }
        //     }
        // });
        // if *component.read() != state {
        //     component.write().update(state);
        // }
        // component
        let input_properties = properties;
        let mut properties = use_signal(|| input_properties.clone());
        let mut component = use_signal(|| Self::from_properties(input_properties.clone()));
        use_effect(capture!(input_properties => move || {
            if properties.read().to_owned() != input_properties {
                *properties.write() = input_properties.clone();
                *component.write() = Self::from_properties(input_properties.clone());
            }
        }));
        component
    }
    fn initialize(&mut self) {

    }

    fn id(&self) -> Option<String> {
        None
    }

    fn on_mount(&mut self) {
    }

    fn on_mount_with_element(&mut self, _element: web_sys::Element) {
    }

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