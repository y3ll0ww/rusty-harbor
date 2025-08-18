extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use syn::{Data, DeriveInput, Fields, Lit, LitStr, Type, parse_macro_input};

#[proc_macro_derive(Harbor, attributes(harbor, header))]
pub fn derive_harbor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // --- Parse #[harbor(url = "...", response = Type)] (or legacy #[response(Type)])
    let mut base_path = String::new();
    let mut response_type: Option<Type> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("harbor") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("url") {
                    let lit: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = lit {
                        base_path = s.value();
                    }
                } else if meta.path.is_ident("response") {
                    let ty: Type = meta.value()?.parse()?;
                    response_type = Some(ty);
                }
                Ok(())
            });
        }
        if attr.path().is_ident("response") {
            match attr.parse_args::<Type>() {
                Ok(ty) => response_type = Some(ty),
                Err(e) => panic!("Invalid #[response(T: DeserializeOwned)] type: {}", e),
            }
        }
    }

    let response_type =
        response_type.expect("Missing #[harbor(response = T)] / #[response(T)] attribute");

    // --- Find placeholders like {field}
    let re = Regex::new(r"\{(\w+)\}").unwrap();
    let placeholder_names: Vec<String> = re
        .captures_iter(&base_path)
        .map(|cap| cap[1].to_string())
        .collect();
    let placeholder_set: HashSet<String> = placeholder_names.iter().cloned().collect();

    // --- Inspect fields: collect header/query code, and remember which fields are Option
    let mut option_map: HashMap<String, bool> = HashMap::new();
    let mut header_push_tokens = Vec::new();
    let mut query_push_tokens = Vec::new();

    if let Data::Struct(data) = &input.data
        && let Fields::Named(fields) = &data.fields
    {
        for field in &fields.named {
            let ident = field
                .ident
                .clone()
                .expect("Harbor derive expects named struct fields");
            let name_str = ident.to_string();

            let is_header = field.attrs.iter().any(|a| a.path().is_ident("header"));
            let is_option = matches!(&field.ty,
                Type::Path(tp) if tp.path.segments.iter().any(|seg| seg.ident == "Option")
            );
            option_map.insert(name_str.clone(), is_option);

            // Header handling (kebab-case header name)
            if is_header {
                let header_name = name_str.replace('_', "-");
                let header_name_lit = LitStr::new(&header_name, Span::call_site());

                if is_option {
                    header_push_tokens.push(quote! {
                        if let Some(v) = self.#ident.as_ref() {
                            headers.push((#header_name_lit.to_string(), v.to_string()));
                        }
                    });
                } else {
                    header_push_tokens.push(quote! {
                        headers.push((#header_name_lit.to_string(), self.#ident.to_string()));
                    });
                }
                continue;
            }

            // Skip query serialization for fields used as path placeholders
            if placeholder_set.contains(&name_str) {
                continue;
            }

            // Query handling (skip None, include Some)
            let key_lit = LitStr::new(&name_str, Span::call_site());
            if is_option {
                query_push_tokens.push(quote! {
                    if let Some(v) = self.#ident.as_ref() {
                        query.push((#key_lit.to_string(), v.to_string()));
                    }
                });
            } else {
                query_push_tokens.push(quote! {
                    query.push((#key_lit.to_string(), self.#ident.to_string()));
                });
            }
        }
    }

    // --- Build path replacement code, with Option-aware placeholder values
    let mut path_replace_tokens = Vec::new();
    for ph in &placeholder_names {
        let ident = format_ident!("{}", ph);
        let ph_lit = LitStr::new(&format!("{{{}}}", ph), Span::call_site());
        let name_lit = LitStr::new(ph, Span::call_site());
        let is_option = *option_map.get(ph).unwrap_or(&false);

        let value_expr = if is_option {
            quote! {
                self.#ident
                    .as_ref()
                    .expect(concat!("Missing required path parameter: ", #name_lit))
                    .to_string()
            }
        } else {
            quote! { self.#ident.to_string() }
        };

        path_replace_tokens.push(quote! {
            path = path.replace(#ph_lit, &{ #value_expr });
        });
    }

    // --- Generate impl
    let expanded = quote! {
        impl HarborRequest for #struct_name {
            type Response = #response_type;

            fn to_url(&self) -> String {
                // Start with base path and apply {placeholders}
                let mut path = #base_path.to_string();
                #(#path_replace_tokens)*

                // Build query from non-header, non-placeholder fields
                let mut query: ::std::vec::Vec<(::std::string::String, ::std::string::String)> = ::std::vec![];
                #(#query_push_tokens)*

                let query_string = ::serde_urlencoded::to_string(&query).unwrap();
                if query_string.is_empty() {
                    path
                } else {
                    format!("{}?{}", path, query_string)
                }
            }

            fn headers(&self) -> Result<HeaderMap, String> {
                let mut header_map = HeaderMap::new();
                let mut headers: ::std::vec::Vec<(::std::string::String, ::std::string::String)> = ::std::vec![];
                #(#header_push_tokens)*

                for (name, value) in headers {
                    let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|e| e.to_string())?;
                    let header_value = HeaderValue::from_bytes(value.as_bytes()).map_err(|e| e.to_string())?;
                    header_map.insert(header_name, header_value);
                }

                Ok(header_map)
            }



        }
    };

    expanded.into()
}
