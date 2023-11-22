use proc_macro2::TokenStream;
use syn::DeriveInput;

pub(crate) mod protoc_oneof;

pub(crate) fn is_protoc_enum(value: &DeriveInput) -> bool {
    value
        .attrs
        .iter()
        .any(|attr| attr.meta.path().is_ident("repr"))
}

pub(crate) trait ImplSerde {
    fn impl_serialize(&self) -> TokenStream;
    fn impl_deserialize(&self) -> TokenStream;
}
