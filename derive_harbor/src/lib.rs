extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{DeriveInput, Lit, Type, parse_macro_input};

#[proc_macro_derive(Harbor, attributes(harbor))]
pub fn derive_query_url(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut base_path = String::new();
    let mut response_type: Option<Type> = None;

    // Extract base path from attribute
    for attr in &input.attrs {
        if attr.path().is_ident("harbor") {
            let _ = attr.parse_nested_meta(|meta| {
                // Handle #[harbor(url = "...")]
                if meta.path.is_ident("url") {
                    let lit: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        base_path = lit_str.value();
                    }
                // Handle #[harbor(response = "...")]
                } else if meta.path.is_ident("response") {
                    let ty: Type = meta.value()?.parse()?;
                    response_type = Some(ty);
                }
                Ok(())
            });
        }

        // Handle #[response(T)]
        if attr.path().is_ident("response") {
            match attr.parse_args::<Type>() {
                Ok(ty) => response_type = Some(ty),
                Err(e) => panic!("Invalid #[response(T: DeserializeOwned)] type: {}", e),
            }
        };
    }

    // Regex to find all placeholders in path like {field}
    let re = Regex::new(r"\{(\w+)\}").unwrap();

    // Find all field names inside {}
    let placeholders: Vec<String> = re
        .captures_iter(&base_path)
        .map(|cap| cap[1].to_string())
        .collect();

    // Build the replacement code for each placeholder
    // We'll generate something like:
    // .replace("{field}", &self.field)
    let mut replaced_path = quote! { #base_path.to_string() };
    for field_name in placeholders {
        let ident = format_ident!("{}", field_name);
        let placeholder = format!("{{{}}}", field_name);
        replaced_path = quote! {
            #replaced_path.replace(#placeholder, &self.#ident.to_string())
        };
    }

    // Define the response type
    let response_type = response_type.expect("Missing #[response(T: DeserializeOwned)] attribute");

    // Generate the final impl with interpolated path
    let expanded = quote! {
        impl HarborRequest for #struct_name {
            type Response = #response_type;

            fn to_url(&self) -> String {
                let path = #replaced_path;
                let query = ::serde_urlencoded::to_string(self).unwrap();
                if query.is_empty() {
                    return path;
                }
                format!("{}?{}", path, query)
            }
        }
    };

    expanded.into()
}
