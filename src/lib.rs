extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use regex::Regex;
use syn::{DataStruct, DeriveInput, Fields};

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
#[proc_macro_derive(Parser, attributes(prompt))]
pub fn parser(input: TokenStream) -> TokenStream {
    let syn_item = syn::parse_macro_input!(input as DeriveInput);

    let name = &syn_item.ident.to_token_stream();

    match syn_item.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            let mut field_getters: Vec<proc_macro2::TokenStream> = vec![];

            let option_regex = Regex::new("Option( )?<.*>").unwrap();

            for field in fields.named.iter() {
                let field_name = &field.ident;
                let mut field_type = field.ty.to_token_stream();

                // remove Option<> from field_type for the getter
                let field_type_str = field_type.to_token_stream().to_string();
                let is_option = option_regex.is_match(field_type_str.as_str());

                if is_option {
                    let type_tokens: Vec<_> = field.ty.to_token_stream().into_iter().collect();
                    let inner_tokens = type_tokens[2..type_tokens.len()-1].to_vec();
                    field_type = proc_macro2::TokenStream::from_iter(inner_tokens);
                }

                let getter_ret: syn::Expr = match is_option {
                    true => syn::parse_quote!(self.#field_name.as_ref().unwrap()),
                    false => syn::parse_quote!(&self.#field_name)
                };
                let field_getter = quote! {
                    pub fn #field_name(&self) -> &#field_type {
                        #getter_ret
                    }
                };

                field_getters.push(field_getter);
            }

            quote! {
                impl #name {
                    #(#field_getters)*   
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
