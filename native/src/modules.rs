mod v0;
mod v1;

use crate::runtime::Runtime;
use wasmi::{Func, Store};

pub type Module = fn(&mut Store<Runtime>) -> (String, String, Func);

pub fn all() -> Vec<Module> {
    let mut vec = vec![];
    vec.extend(v0::modules::modules());
    vec.extend(v1::modules::modules());
    vec
}
