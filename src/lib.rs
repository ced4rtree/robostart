extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use regex::Regex;
use syn::{Attribute, DataStruct, DeriveInput, Fields, FieldsNamed, Meta, Type};

fn is_option(ty: &Type) -> bool {
    let option_regex = Regex::new("Option( )?<.*>").unwrap();
    let field_type_str = ty.to_token_stream().to_string();
    option_regex.is_match(field_type_str.as_str())
}

/// Generate code for getting parser fields. Useful for bypassing the Option<T>
/// automatically, which is necessary to mark an argument as optional for clap,
/// but unnecessary within Robostart since we prompt for any missing info.
fn parser_gen_getters(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|field| {
        let field_name = &field.ident;
        let mut field_type = field.ty.to_token_stream();

        // remove Option<> from field_type for the getter
        let is_option = is_option(&field.ty); 
        if is_option {
            let type_tokens: Vec<_> = field.ty.to_token_stream().into_iter().collect();
            let inner_tokens = type_tokens[2..type_tokens.len()-1].to_vec();
            field_type = proc_macro2::TokenStream::from_iter(inner_tokens);
        }

        let getter_ret: syn::Expr = match is_option {
            true => syn::parse_quote!(self.#field_name.as_ref().unwrap()),
            false => syn::parse_quote!(&self.#field_name)
        };
        quote! {
            pub fn #field_name(&self) -> &#field_type {
                #getter_ret
            }
        }
    }).collect()
}

/// Generate all the code that will run if an optional value is not present.
/// It is assumed that the absent handler will populate the data.
fn parser_gen_absent_handlers(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().filter_map(|field| {
        let ident = &field.ident;
        let is_option = is_option(&field.ty);

        // don't need to handle a non-optional field
        if !is_option {
            return None;
        }

        let handler_attrs: Vec<&Attribute> = field
            .attrs
            .iter()
            .filter(|field| field.path().is_ident("absent_handler"))
            .collect();
        let handler = match handler_attrs.len() {
            ..=0 => panic!("All elements of type Option<T> in a struct that derives robostart::Parser must define an absent handler through the #[absent_handler(...)] attribute"),
            1 => handler_attrs[0],
            2.. => panic!("You may not define multiple absent handlers for one element in a struct that derives robostart::Parser"),
        };

        let handler_tokens = match &handler.meta {
            Meta::List(lst) => &lst.tokens,
            _ => panic!("absent_handler helper macro should be assigned in list form, e.g. #[absent_handler(...)]. Found #[{:?}] instead.", handler), 
        };

        Some(quote! {
            if self.#ident.is_none() {
                self.#ident = Some(#handler_tokens)
            }
        })
    }).collect()
}

/// Special Robostart parser
///
/// Automatically generates getters for all elements in a struct, returning
/// self.<elem>.as_ref().unwrap() if the element is of type Option<T>.
///
/// Designed for Robostart's use case of having all options capable of being
/// input through CLI args while prompting the user for anything not submitted
/// that way. This enables a single struct definition for clap arguments while
/// reducing boilerplate around navigating the necessary Option<> handling that
/// comes with that.
///
/// # Examples
///
/// ```
/// #[derive(robostart::Parser, clap::Parser)]
/// struct CliParser {
///     #[arg(short, long)]
///     #[absent_handler(prompt_u32())]
///     arg1: Option<u32>
///
///     #[arg(short, long)]
///     arg2: String
/// }
///
/// impl CliParser {
///     pub fn print_args(&self) {
///         // Getters are automatically generated for arg1 and arg2.
///         // The getters for optional values assume a value has been populated,
///         // as Robostart will prompt the user for any absent values.
///         println!("arg1: {}, arg2: {}", self.arg1(), self.arg2());
///     }
/// }
/// ```
#[proc_macro_derive(Parser, attributes(absent_handler))]
pub fn parser(input: TokenStream) -> TokenStream {
    let syn_item = syn::parse_macro_input!(input as DeriveInput);

    let name = &syn_item.ident.to_token_stream();

    match syn_item.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            let field_getters = parser_gen_getters(fields);
            let absent_handlers = parser_gen_absent_handlers(fields);

            quote! {
                impl #name {
                    #(#field_getters)*

                    pub fn handle_absent_values(&mut self) {
                        #(#absent_handlers)*
                    }
                }
            }.into()
        }
        _ => {
            panic!("Parser macro can only be used with structs that have named elements.");
        }
    }
}

#[proc_macro_derive(AllVariants)]
pub fn derive_all_variants(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();

    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => {
            enum_item.variants.into_iter().map(|v| v.ident)
        }
        _ => panic!("AllVariants only works on enums"),
    };
    let enum_name = syn_item.ident;

    let expanded = quote! {
        impl #enum_name {
            pub fn all_variants() -> &'static[#enum_name] {
                &[ #(#enum_name::#variants),* ]
            }
        }
    };
    expanded.into()
}
