pub mod v0;
pub mod v1;

#[cfg(not(feature = "bindings"))]
use crate::runtime::Runtime;
#[cfg(not(feature = "bindings"))]
use wasmi::{Func, Store};

#[cfg(not(feature = "bindings"))]
pub type Module = fn(&mut Store<Runtime>) -> (String, String, Func);
