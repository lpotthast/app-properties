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
        #[derive(Debug, snafu::Snafu)]
        pub enum #error_type {
            #[snafu(display("AppPropertiesError: Unable to deserialize YAML string."))]
            DeserializeYaml {
                source: serde_yaml::Error,
                backtrace: snafu::Backtrace,
            }
        }

        impl app_properties::AppPropertiesExt for #ident {
            type Error = #error_type;

            #[tracing::instrument(level = "info", name = "load_properties")]
            fn load() -> Result<Self, Self::Error> {
                use snafu::ResultExt;

                tracing::info!(src_path = #src_path, "Using source");
                let serialized = include_str!(#src_path);
                let mut deserialized: #raw_ident = serde_yaml::from_str(serialized)
                    .context(DeserializeYamlSnafu {})?;
                Ok(deserialized.replace_env(replace_env::Metadata { secret: false }).into())
            }
        }
    }
    .into()
}
