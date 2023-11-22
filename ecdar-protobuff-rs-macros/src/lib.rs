use proc_macro::TokenStream as TS;
use quote::quote;
use syn::DeriveInput;

mod to_serde;
use to_serde::*;

#[proc_macro_attribute]
pub fn serde_derive(_: TS, input: TS) -> TS {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let header = match &input.data {
        syn::Data::Enum(_) if !to_serde::is_protoc_enum(&input) => {
            quote! {
                #[derive(ecdar_protobuff_rs_macros::ProtocOneof)]
            }
        }
        syn::Data::Enum(_) if to_serde::is_protoc_enum(&input) => {
            quote! {
                #[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
            }
        }
        syn::Data::Struct(_) => {
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                #[serde(rename_all = "camelCase")]
            }
        }
        _ => {
            panic!("this macro is only for structs and enums")
        }
    };

    let out = quote! {
        #header
        #input
    };

    out.into()
}

#[proc_macro_derive(ProtocOneof)]
pub fn derive_protoc_oneof(input: TS) -> TS {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let to_serde: crate::to_serde::protoc_oneof::ProtocOneof = input.into();
    let serialize = to_serde.impl_serialize();
    let deserialize = to_serde.impl_deserialize();

    quote! {
        #serialize
        #deserialize
    }
    .into()
}

