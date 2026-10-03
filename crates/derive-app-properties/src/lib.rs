#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(app_properties), supports(struct_any))]
struct MyInputReceiver {
    ident: syn::Ident,

    src: String,
}

#[proc_macro_derive(AppProperties, attributes(app_properties))]
#[proc_macro_error]
pub fn store(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let input: MyInputReceiver = match FromDeriveInput::from_derive_input(&ast) {
        Ok(args) => args,
        Err(err) => return darling::Error::write_errors(err).into(),
    };

    let ident = &input.ident;
    let raw_ident = Ident::new(format!("Raw{}", ident).as_str(), Span::call_site());

    let error_type = Ident::new(format!("{}Error", ident).as_str(), Span::call_site());

    // We currently only support one option: Loading the source file with the include_str! macro.
    let src_path = input.src;

    quote! {
        #[derive(Debug)]
        pub enum #error_type {
            DeserializeYaml { source: serde_yaml::Error },
        }

        impl std::fmt::Display for #error_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::DeserializeYaml { .. } => {
                        write!(f, "AppPropertiesError: Unable to deserialize YAML string.")
                    }
                }
            }
        }

        impl std::error::Error for #error_type {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    Self::DeserializeYaml { source } => Some(source),
                }
            }
        }

        impl app_properties::AppPropertiesExt for #ident {
            type Error = #error_type;

            #[tracing::instrument(level = "info", name = "load_properties")]
            fn load() -> Result<Self, Self::Error> {
                tracing::info!(src_path = #src_path, "Using source");
                let serialized = include_str!(#src_path);
                let mut deserialized: #raw_ident = serde_yaml::from_str(serialized)
                    .map_err(|source| #error_type::DeserializeYaml { source })?;
                let replaced: #raw_ident = replace_env::ReplaceEnv::replace_env(deserialized, replace_env::Metadata { secret: false });
                let as_ident: #ident = replaced.into();
                Ok(as_ident)
            }
        }
    }
    .into()
}
