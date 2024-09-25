use crate::prelude::*;

pub trait WebComponent: Sized + Clone + PartialEq + 'static {
    type State;

    fn instance(state: Self) -> Signal<Self> {
        let mut component = use_signal(|| {
            let mut state = state.clone();
            state.initialize();
            state
        });
        use_effect(move || {
            component.write().on_mount();
            let id = component.write().id().clone();
            if let Some(id) = id {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&id) {
                            component.write().on_mount_with_element(element);
                        }
                    }
                }
            }
        });
        if *component.read() != state {
            component.write().update(state);
        }
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
    
    fn render(component: Signal<Self>, state: Signal<Self::State>) -> Element;
}

#[macro_export]
macro_rules! expose_component {
    ($props:ident as $name:ident) => {
        #[component]
        pub fn $name(props: $props) -> Element {
            let component = $props::instance(props);
            let state = use_signal(|| Default::default());
            WebComponent::render(component, state)
        }
    };
}