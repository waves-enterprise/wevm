use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn module(attr: TokenStream2, item: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let mut bindings: Vec<TokenStream2> = vec![];
    let mut modules: Vec<TokenStream2> = vec![];

    let env = attr.to_string();

    let input = syn::parse2::<syn::ItemMod>(item)?;

    if let Some(content) = input.content {
        for item in content.1 {
            if let syn::Item::Fn(func) = item {
                let func_name = &func.sig.ident;
                let func_block = &func.block;

                let mut bindings_inputs: Vec<TokenStream2> = vec![];
                let mut modules_inputs: Vec<TokenStream2> = vec![];

                for arg in func.sig.inputs.iter() {
                    if let syn::FnArg::Typed(a) = arg {
                        if let syn::Pat::Ident(pat_ident) = &*a.pat {
                            let arg_name = &pat_ident.ident;

                            if let Some(type_string) = parse_type(&a.ty, true) {
                                bindings_inputs.push(quote!(
                                    #arg_name: #type_string
                                ));
                            }

                            if let Some(type_string) = parse_type(&a.ty, false) {
                                modules_inputs.push(quote!(
                                    #arg_name: #type_string
                                ));
                            }
                        }
                    }
                }

                let mut bindings_output: Vec<TokenStream2> = vec![];
                let mut modules_output: Vec<TokenStream2> = vec![];

                if let syn::ReturnType::Type(_, ty) = func.sig.output {
                    if let Some(type_string) = parse_type(&ty, true) {
                        bindings_output.push(quote!(#type_string));
                    }

                    if let Some(type_string) = parse_type(&ty, false) {
                        modules_output.push(quote!(#type_string));
                    }
                }

                bindings.push(quote!(
                    #[no_mangle]
                    pub fn #func_name( #( #bindings_inputs ),* ) -> ( #( #bindings_output ),* );
                ));

                let module = attr.to_string();
                let name = func_name.to_string();

                modules.push(quote!(
                    fn #func_name(store: &mut Store<Runtime>) -> (String, String, Func) {
                        (#module.to_string(), #name.to_string(), Func::wrap(
                            store,
                            |caller: Caller<Runtime>, #( #modules_inputs ),* | -> ( #( #modules_output ),* ) {
                                let func = #func_block;
                                func(caller)
                            }
                        ))
                    }

                    vec.push(#func_name);
                ));
            }
        }
    }

    Ok(quote!(
        #[cfg(feature = "bindings")]
        pub mod bindings {
            #[link(wasm_import_module = #env)]
            extern "C" {
                #( #bindings )*
            }
        }

        #[cfg(not(feature = "bindings"))]
        pub mod modules {
            use crate::{env, error::{Error, RuntimeError}, modules::Module, runtime::Runtime};
            use wasmi::{Caller, Func, Store};

            pub fn modules() -> Vec<Module> {
                let mut vec: Vec<Module> = vec![];

                #( #modules )*

                vec
            }
        }
    ))
}

fn parse_type(type_: &syn::Type, is_bindings: bool) -> Option<TokenStream2> {
    match (type_, is_bindings) {
        (syn::Type::Ptr(_), true) => Some(quote!(*const u8)),
        (syn::Type::Ptr(_), false) => Some(quote!(u32)),
        (syn::Type::Path(type_path), _) => {
            let path_seg = &type_path.path.segments[0];
            match path_seg.ident.to_string().as_str() {
                "usize" => Some(quote!(u32)),
                "bool" => Some(quote!(i32)),
                _ => Some(quote!(#type_path)),
            }
        }
        (syn::Type::Tuple(type_tuple), is_bindings) => {
            let mut result: Vec<TokenStream2> = vec![];

            for item in &type_tuple.elems {
                if let Some(type_string) = parse_type(item, is_bindings) {
                    result.push(type_string);
                }
            }

            Some(quote!(#( #result ),*))
        }
        _ => None,
    }
}
