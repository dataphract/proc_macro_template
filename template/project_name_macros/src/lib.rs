extern crate proc_macro;

#[proc_macro]
pub fn function_like_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[proc_macro_derive(TraitName, attributes(attr_name))]
pub fn derive_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[proc_macro_attribute]
pub fn attribute_macro(
    _attr: proc_macro::TokenStream,
    _item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    todo!()
}
