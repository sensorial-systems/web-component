use std::{future::Future, sync::{Arc, Mutex, MutexGuard}};

use dioxus::signals::{Readable, Signal, Writable};
use enum_as_inner::EnumAsInner;

#[derive(Clone, Default)]
pub struct Resource<T> {
    state: Arc<Mutex<ResourceState<T>>>
}

#[derive(Clone, EnumAsInner)]
pub enum ResourceState<T> {
    Unloaded,
    Loading,
    Loaded(T)
}

impl<T> Default for ResourceState<T> {
    fn default() -> Self {
        Self::Unloaded
    }
}

impl<T> Resource<T> {
    pub fn new(future: impl Future<Output = T> + 'static) -> Self
    where T: 'static
    {
        let state = Default::default();
        let mut resource = Self { state };
        resource.load(future);
        resource
    }

    pub fn get(&self) -> Option<T>
    where T: Clone
    {
        match self.state.try_lock().unwrap().clone() {
            ResourceState::Loaded(value) => Some(value),
            _ => None
        }
    }

    pub fn clear(&mut self) {
        *self.state.lock().unwrap() = ResourceState::Unloaded;
    }

    pub fn is_unloaded(&self) -> bool {
        self.state.lock().unwrap().is_unloaded()
    }

    pub fn get_state(&self) -> MutexGuard<ResourceState<T>> {
        self.state.lock().unwrap()
    }

    pub fn load(&mut self, future: impl Future<Output = T> + 'static)
    where T: 'static
    {
        let state = self.state.clone();
        *state.lock().unwrap() = ResourceState::Loading;
        wasm_bindgen_futures::spawn_local(async move {
            *state.lock().unwrap() = ResourceState::Loaded(future.await);
        })
    }

    pub fn load_and_notify<C>(&mut self, mut component: Signal<C>, future: impl Future<Output = T> + 'static)
    where T: 'static
    {
        let state = self.state.clone();
        *state.lock().unwrap() = ResourceState::Loading;
        wasm_bindgen_futures::spawn_local(async move {
            *state.lock().unwrap() = ResourceState::Loaded(future.await);
            component.write();
        })
    }
}

pub trait ResourceTrait<Component> {
    fn update_resource<Type>(&self, f: impl FnOnce(&Component) -> &Resource<Type>, future: impl Future<Output = Type> + 'static)
    where Type: Clone + 'static;

    fn acquire_resource<Type>(&self, f: impl FnMut(&Component) -> &Resource<Type>, future: impl Future<Output = Type> + 'static) -> Option<Type>
    where Type: Clone + 'static;
}

impl<Component> ResourceTrait<Component> for Signal<Component> {
    fn update_resource<Type>(&self, f: impl FnOnce(&Component) -> &Resource<Type>, future: impl Future<Output = Type> + 'static)
    where Type: Clone + 'static
    {
        let mut resource = f(&*self.read()).clone();
        resource.load_and_notify(*self, future);
    }

    fn acquire_resource<Type>(&self, mut f: impl FnMut(&Component) -> &Resource<Type>, future: impl Future<Output = Type> + 'static) -> Option<Type>
        where Type: Clone + 'static
    {
        let resource = f(&*self.read()).clone();
        if resource.is_unloaded() {
            self.update_resource(f, future);
        }
        resource.get()
    }
}
