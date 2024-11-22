extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Ident};

#[proc_macro_derive(DisplayViaDebug)]
pub fn display_via_debug_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the name of the struct or enum
    let name = input.ident;

    // Generate the implementation of Display using Debug
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };

    // Hand the generated impl back to the compiler
    TokenStream::from(expanded)
}

// partial_macro/src/lib.rs


// Declare the attribute macro
#[proc_macro_attribute]
pub fn make_partial(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as DeriveInput);

    // Get the name (identifier) of the input struct
    let struct_ident = input.ident.clone();

    // Create a new identifier for the partial struct by prefixing "Partial"
    let partial_struct_ident = Ident::new(
        &format!("Partial{}", struct_ident),
        struct_ident.span(),
    );

    // Extract generics (lifetimes, type parameters, where clauses)
    let generics = input.generics.clone();

    // Extract attributes and filter out the `make_partial` attribute
    let _attrs = filter_make_partial_attr(&input.attrs);
    // let attrs = &input.attrs;

    // Ensure the input is a struct
    if let syn::Data::Struct(data_struct) = &input.data {
        // Process each field in the struct
        let fields = data_struct.fields.iter().map(|field| {
            let ident = field.ident.clone();
            let ty = field.ty.clone();
            let field_attrs = field.attrs.clone(); // Preserve field attributes
            quote! {
                #(#field_attrs)*
                pub #ident: Option<#ty>,
            }
        });

        // Generate the complete code for both structs
        let gen = quote! {
            // Include the original struct unchanged
            #input

            // Define the new partial struct with attributes and generics
            #[derive(Debug, DisplayViaDebug, Serialize, Deserialize, Clone)]
            // #(#attrs)*
            pub struct #partial_struct_ident #generics {
                #(#fields)*
            }
        };
        // Return the generated code as a TokenStream
        gen.into()
    } else {
        // If the input is not a struct, emit a compiler error
        panic!("make_partial can only be used with structs");
    }
}

// Function to filter out the `make_partial` attribute
fn filter_make_partial_attr(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| {
            // Check if the attribute is not `make_partial`
            !is_make_partial_attr(attr)
        })
        .cloned()
        .collect()
}

// Helper function to check if an attribute is `make_partial`
fn is_make_partial_attr(attr: &Attribute) -> bool {
    attr.path()
        .get_ident()
        .map(|ident| ident == "make_partial")
        .unwrap_or(true)
}

