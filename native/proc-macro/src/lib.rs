mod generator;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    match generator::module(attr.into(), item.into()) {
        Ok(result) => result.into(),
        Err(_) => panic!(),
    }
}
