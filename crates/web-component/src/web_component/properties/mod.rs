use crate::prelude::*;

pub type NoProperties = ();

pub trait FromProperties<P: Properties>: Sized {
    fn from_properties(properties: P) -> Self;
}

impl<T> FromProperties<NoProperties> for T
where
    T: Default + WebComponent<Properties = NoProperties>,
{
    fn from_properties(_: ()) -> Self {
        Default::default()
    }
}
