pub use dioxus;
pub use dioxus::prelude::*;

pub use crate::{WebComponent, Resource, ResourceTrait, FromProperties, NoProperties};
pub use crate::{capture, capture_async, event, expose_component};
pub use crate::logger::tracing as log;
pub use wasm_bindgen_futures;