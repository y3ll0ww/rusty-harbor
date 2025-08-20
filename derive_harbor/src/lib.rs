extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use regex::Regex;
use std::collections::HashSet;
use syn::{Data, DeriveInput, Fields, Lit, LitStr, Type, parse_macro_input};

#[proc_macro_derive(Harbor, attributes(harbor, header, response))]
pub fn derive_harbor(input: TokenStream) -> TokenStream {
    match derive_harbor_impl(parse_macro_input!(input as DeriveInput)) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_harbor_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = &input.ident;

    // --- Parse #[harbor(url = "...", response = Type)] or #[response(Type)]
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
            if let Ok(ty) = attr.parse_args::<Type>() {
                response_type = Some(ty);
            }
        }
    }

    let response_type =
        response_type.expect("Missing #[harbor(response = T)] or #[response(T)] attribute");

    // --- Find placeholders {field}
    let re = Regex::new(r"\{(\w+)\}").unwrap();
    let placeholders: Vec<String> = re
        .captures_iter(&base_path)
        .map(|cap| cap[1].to_string())
        .collect();
    let placeholder_set: HashSet<String> = placeholders.iter().cloned().collect();

    // --- Inspect fields
    let mut header_push_tokens = Vec::new();
    let mut query_push_tokens = Vec::new();

    if let Data::Struct(data) = &input.data
        && let Fields::Named(fields) = &data.fields
    {
        for field in &fields.named {
            let ident = field.ident.clone().unwrap();
            let name_str = ident.to_string();

            let mut header_name: Option<String> = None;
            let mut is_header = false;

            for attr in &field.attrs {
                if attr.path().is_ident("header") {
                    is_header = true;

                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename") {
                            let lit: Lit = meta.value()?.parse()?;
                            if let Lit::Str(s) = lit {
                                header_name = Some(s.value());
                            } else {
                                return Err(syn::Error::new_spanned(
                                    lit,
                                    "expected string literal for #[header(rename = ...)]",
                                ));
                            }
                            Ok(())
                        } else {
                            Err(syn::Error::new_spanned(
                                meta.path,
                                "only #[header] or #[header(rename = \"...\")] is allowed",
                            ))
                        }
                    })?;
                }
            }

            if is_header {
                let header_name = header_name.unwrap_or_else(|| name_str.replace('_', "-"));
                let header_name_lit = LitStr::new(&header_name, Span::call_site());

                let is_option = matches!(&field.ty,
                    Type::Path(tp) if tp.path.segments.iter().any(|seg| seg.ident == "Option")
                );

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
            } else if !placeholder_set.contains(&name_str) {
                // Not a header, not a path placeholder => query param
                let is_option = matches!(&field.ty,
                    Type::Path(tp) if tp.path.segments.iter().any(|seg| seg.ident == "Option")
                );

                if is_option {
                    query_push_tokens.push(quote! {
                        if let Some(v) = self.#ident.as_ref() {
                            query.push((#name_str.to_string(), v.to_string()));
                        }
                    });
                } else {
                    query_push_tokens.push(quote! {
                        query.push((#name_str.to_string(), self.#ident.to_string()));
                    });
                }
            }
        }
    }

    // --- Path replacement
    let mut path_replace_tokens = Vec::new();
    for ph in &placeholders {
        let ident = format_ident!("{}", ph);
        let ph_lit = LitStr::new(&format!("{{{}}}", ph), Span::call_site());
        path_replace_tokens.push(quote! {
            path = path.replace(#ph_lit, &self.#ident.to_string());
        });
    }

    // --- Impl
    let expanded = quote! {
        impl HarborRequest for #struct_name {
            type Response = #response_type;

            fn to_url(&self) -> String {
                let mut path = #base_path.to_string();
                #(#path_replace_tokens)*

                let mut query: Vec<(String, String)> = Vec::new();
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
                let mut headers: Vec<(String, String)> = Vec::new();
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

    Ok(expanded)
}
