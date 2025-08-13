extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{DeriveInput, Lit, parse_macro_input};

#[proc_macro_derive(QueryUrl, attributes(query_url))]
pub fn derive_query_url(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Extract base path from attribute #[query_url(path = "...")]
    let mut base_path = String::new();
    for attr in &input.attrs {
        if attr.path().is_ident("query_url") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("path") {
                    let lit: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        base_path = lit_str.value();
                    }
                }
                Ok(())
            });
        }
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

    // Generate the final impl with interpolated path
    let url_encoded = quote! {
        impl HarborRequest for #struct_name {
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

    url_encoded.into()
}
