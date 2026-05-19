extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use regex::Regex;
use syn::{Attribute, DataStruct, DeriveInput, Fields, FieldsNamed, Meta, Type};

fn is_once_cell(ty: &Type) -> bool {
    let option_regex = Regex::new("OnceCell( )?<.*>").unwrap();
    let field_type_str = ty.to_token_stream().to_string();
    option_regex.is_match(field_type_str.as_str())
}

/// Generate field getters, lazily handling absent values as they pop up
fn lazy_struct_gen_getters(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named.iter().map(|field| {
        let field_name = &field.ident;

        // remove OnceCell<> from field_type for the getter
        if !is_once_cell(&field.ty) {
            panic!(
                "All fields in a struct that derives robostart::Config should be of type OnceCell<T> in order to track internal state. The following field is not: {}",
                field_name.to_token_stream()
            );
        }
        let type_tokens: Vec<_> = field.ty.to_token_stream().into_iter().collect();
        let inner_tokens = type_tokens[2..type_tokens.len()-1].to_vec();
        let field_type = proc_macro2::TokenStream::from_iter(inner_tokens);

        // find handler code
        let handler_attrs: Vec<&Attribute> = field
            .attrs
            .iter()
            .filter(|field| field.path().is_ident("absent_handler"))
            .collect();
        let handler = match handler_attrs.len() {
            ..=0 => panic!("All elements of type OnceCell<T> in a struct that derives robostart::Parser must define an absent handler through the #[absent_handler(...)] attribute"),
            1 => handler_attrs[0],
            2.. => panic!("You may not define multiple absent handlers for one element in a struct that derives robostart::Parser"),
        };

        let handler_tokens = match &handler.meta {
            Meta::List(lst) => &lst.tokens,
            _ => panic!("absent_handler helper macro should be assigned in list form, e.g. #[absent_handler(...)]. Found #[{:?}] instead.", handler), 
        };

        quote! {
            pub fn #field_name(&self) -> ::anyhow::Result<&#field_type> {
                if (self.#field_name.get().is_none()) {
                    let handler = #handler_tokens;
                    let value = handler(&self)?;
                    return Ok(self.#field_name.get_or_init(|| value));
                }
                Ok(self.#field_name.get().unwrap())
            }
        }
    }).collect()
}

/// Special Robostart parser
///
/// Automatically generate getters for all values in a struct while also
/// enforcing that all fields are of the type OnceCell<T> in order to make the
/// fields lazily evaluated, i.e. the data inside of the OnceCell will be
/// populated with the return value of the absent_handler when the data is first
/// retrieved, and that data is then cached for future use.
///
/// # Examples
///
/// ```
/// #[derive(robostart::LazyStruct)]
/// struct Example {
///     #[absent_handler(|example_inst|: &Self| -> Result<u32, Box<dyn std::error::Error>> {
///         println!("Arg2 is {}", example_inst.arg2());
///         Ok(prompt_u32()) // absent_handlers must return a Result<T, E>
///     })]
///     arg1: OnceCell<u32>
///
///     #[absent_handler(|_| Ok("no-op default value".to_string()))]
///     arg2: OnceCell<String>
/// }
///
/// impl Config {
///     pub fn print_args(&self) {
///         // Getters are automatically generated for arg1 and arg2.
///         // If this is the first time an arg is being called upon, the code for it's absent
///         // handler will be called, and the value will be found from it. Future invocations will
///         // use that cached value.
///         assert_eq!(self.arg1(), 3); // 'Arg2 is "no-op default value"' is printed
///         assert_eq!(self.arg1(), 3); // Nothing is printed
///     }
/// }
/// ```
#[proc_macro_derive(LazyStruct, attributes(absent_handler))]
pub fn derive_lazy_struct(input: TokenStream) -> TokenStream {
    let syn_item = syn::parse_macro_input!(input as DeriveInput);

    let name = &syn_item.ident.to_token_stream();

    match syn_item.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            let field_getters = lazy_struct_gen_getters(fields);
            let field_inits: Vec<proc_macro2::TokenStream> = fields.named.iter().map(|field| {
                let ident = &field.ident;
                quote!(#ident: OnceCell::new(),)
            }).collect();

            quote! {
                impl #name {
                    #(#field_getters)*

                    pub fn new() -> ::anyhow::Result<Self> {
                        Ok(#name {
                            #(#field_inits)*
                        })
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
