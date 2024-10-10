use std::{future::Future, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};

use dioxus::{prelude::spawn, signals::{Readable, Signal, Writable}};
use enum_as_inner::EnumAsInner;

#[derive(Clone)]
pub struct Resource<T> {
    state: Arc<Mutex<ResourceState<T>>>
}

impl<T> Default for Resource<T> {
    fn default() -> Self {
        let state = Default::default();
        Self { state }
    }
}

#[derive(Clone, EnumAsInner)]
pub enum ResourceState<T> {
    Unloaded,
    Loading,
    Unavailable,
    Loaded(T)
}

impl<T> Default for ResourceState<T> {
    fn default() -> Self {
        Self::Unloaded
    }
}

impl<T> Resource<T> {
    pub fn new(future: impl Future<Output = Option<T>> + 'static) -> Self
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
        *self.state.blocking_lock() = ResourceState::Unloaded;
    }

    pub fn is_unloaded(&self) -> bool {
        self.state.blocking_lock().is_unloaded()
    }

    pub fn get_state(&self) -> MutexGuard<ResourceState<T>> {
        self.state.blocking_lock()
    }

    pub fn load(&mut self, future: impl Future<Output = Option<T>> + 'static)
    where T: 'static
    {
        let state = self.state.clone();
        *state.blocking_lock() = ResourceState::Loading;
        spawn(async move {
            *state.lock().await = future
                .await
                .map(ResourceState::Loaded)
                .unwrap_or(ResourceState::Unavailable);
        });
    }

    pub fn load_and_notify<C>(&mut self, mut component: Signal<C>, future: impl Future<Output = Option<T>> + 'static)
    where T: 'static
    {
        let state = self.state.clone();
        *state.blocking_lock() = ResourceState::Loading;
        spawn(async move {
            *state.lock().await = future
                .await
                .map(ResourceState::Loaded)
                .unwrap_or(ResourceState::Unavailable);
            component.write();
        });
    }

    pub fn as_loaded(&self) -> Option<T>
    where T: Clone
    {
        self.state.blocking_lock().as_loaded().cloned()
    }
}

pub trait ResourceTrait<Component> {
    fn update_resource<Type>(&self, f: impl FnOnce(&Component) -> &Resource<Type>, future: impl Future<Output = Option<Type>> + 'static)
    where Type: Clone + 'static;

    fn acquire_resource<Type>(&self, f: impl FnMut(&Component) -> &Resource<Type>, future: impl Future<Output = Option<Type>> + 'static) -> ResourceState<Type>
    where Type: Clone + 'static;

    fn web_get<Type>(&self, f: impl FnMut(&Component) -> &Resource<Type>, url: String) -> ResourceState<Type>
    where Type: Clone + 'static + serde::de::DeserializeOwned;

    fn web_post<Type>(&self, f: impl FnMut(&Component) -> &Resource<Type>, url: String, body: impl serde::Serialize + 'static) -> ResourceState<Type>
    where Type: Clone + 'static + serde::de::DeserializeOwned;
}

impl<Component> ResourceTrait<Component> for Signal<Component> {
    fn update_resource<Type>(&self, f: impl FnOnce(&Component) -> &Resource<Type>, future: impl Future<Output = Option<Type>> + 'static)
    where Type: Clone + 'static
    {
        let mut resource = f(&*self.read()).clone();
        resource.load_and_notify(self.clone(), future);
    }

    fn acquire_resource<Type>(&self, mut f: impl FnMut(&Component) -> &Resource<Type>, future: impl Future<Output = Option<Type>> + 'static) -> ResourceState<Type>
        where Type: Clone + 'static
    {
        let resource = f(&*self.read()).clone();
        if resource.is_unloaded() {
            self.update_resource(f, future);
        }
        let x = resource.get_state().clone();
        x
    }

    fn web_get<Type>(&self, mut f: impl FnMut(&Component) -> &Resource<Type>, url: String) -> ResourceState<Type>
    where Type: Clone + 'static + serde::de::DeserializeOwned
    {
        let resource = f(&*self.read()).clone();
        if resource.is_unloaded() {
            self.update_resource(f, async move {
                reqwest::get(url)
                    .await
                    .ok()?
                    .json::<Type>()
                    .await
                    .ok()
            });
        }
        let x = resource.get_state().clone();
        x
        }

    fn web_post<Type>(&self, mut f: impl FnMut(&Component) -> &Resource<Type>, url: String, body: impl serde::Serialize + 'static) -> ResourceState<Type>
    where Type: Clone + 'static + serde::de::DeserializeOwned
    {
        let resource = f(&*self.read()).clone();
        if resource.is_unloaded() {
            self.update_resource(f, async move {
                reqwest::Client::new()
                    .post(url)
                    .json(&body)
                    .send()
                    .await
                    .ok()?
                    .json::<Type>()
                    .await
                    .ok()
            });
        }
        let x = resource.get_state().clone();
        x
    }
}
