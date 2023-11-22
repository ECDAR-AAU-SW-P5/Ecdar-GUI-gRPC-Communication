use super::*;
use convert_case::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) struct ProtocOneof {
    ast: DeriveInput,
}

impl ImplSerde for ProtocOneof {
    fn impl_serialize(&self) -> TokenStream {
        let ident = &self.ast.ident;
        let variants = if let Data::Enum(data) = &self.ast.data {
            (&data.variants)
                .into_iter()
                .map(|v| {
                    let ident = &v.ident;
                    let ident_str = v.ident.to_string().to_case(Case::Camel);
                    quote! {
                        Self::#ident(value) => {
                            map.serialize_entry("oneofKind", #ident_str)?;
                            map.serialize_entry(#ident_str, value.into())?;
                        }
                    }
                })
                .collect::<Vec<_>>()
        } else {
            panic!("this macro only applies to enums")
        };

        quote! {
            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer : S) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::ser::Serializer {
                    use serde::ser::SerializeMap;
                    let mut map = serializer.serialize_map(Some(2))?;
                    match self {
                        #(#variants),*
                    };
                    map.end()
                }
            }
        }
    }

    fn impl_deserialize(&self) -> TokenStream {
        let ident = &self.ast.ident;
        let visitor = self.create_visitor();
        quote! {
            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>{
                    #visitor
                    deserializer.deserialize_map(Visitor)
                }
            }
        }
    }
}

impl ProtocOneof {
    pub fn create_visitor(&self) -> TokenStream {
        let ident = &self.ast.ident;
        let variants = if let Data::Enum(data) = &self.ast.data {
            data.variants.iter().map(|v| {
                let ident = &v.ident;
                let ident_str = &v.ident.to_string().to_case(Case::Camel);
                let variant_type = if let syn::Fields::Unnamed(field) = &v.fields {
                    &field
                        .unnamed
                        .iter()
                        .next()
                        .expect("Filds must contain a type")
                        .ty
                } else {
                    panic!(
                        "This macro only works on enums where all attrimutes is a touble attribute"
                    )
                };
                quote! {
                    #ident_str => {
                        let (_type_name, value) = map.next_entry::<&str, #variant_type>()?.unwrap();
                        Ok(Self::Value::#ident(value.into()))
                    }
                }
            })
        } else {
            panic!("This macro only works on enums")
        };
        quote! {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = #ident;
                fn expecting(&self, formatter : &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("failed to parse")
                }

                fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de> {
                    let (oneof, type_oneof) = map.next_entry::<&str, &str>()?.unwrap();
                    match oneof {
                        #(#variants),*,
                        _ => {
                            todo!()
                        }
                    }
                }
            }
        }
    }
}

impl From<DeriveInput> for ProtocOneof {
    fn from(value: DeriveInput) -> Self {
        Self { ast: value }
    }
}
