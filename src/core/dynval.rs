use downcast::{downcast, Any};
use dyn_clone::{clone_trait_object, DynClone};
use std::fmt;

/// Any trait with downcast, clone and base any impl
pub trait Value: DynClone + Any {}
clone_trait_object!(Value);
downcast!(dyn Value);

impl<T: Clone + Any> Value for T {}

impl fmt::Debug for dyn Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value").finish_non_exhaustive()
    }
}
