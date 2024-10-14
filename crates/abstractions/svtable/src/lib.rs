//! # Static Virtual Table (svtable) Macro
//!
//! This crate provides the `svtable` macro, which generates a static virtual table (vtable) for traits.
//! The macro simplifies the creation of vtables, which can be useful for optimizing dynamic dispatch
//! or implementing trait objects with custom layouts.
//!
//! ## Usage and Testing
//!
//! Here's a basic example of how to use the `svtable` macro and test its output:
//!
//! ```rust
//! use svtable::svtable;
//!
//! #[svtable]
//! pub trait A {
//!     fn assoc_fn1();
//!     fn assoc_fn2(x: i32) -> bool;
//! }
//!
//! // Implement the trait
//! struct Implementor;
//!
//! impl A for Implementor {
//!     fn assoc_fn1() {
//!         println!("Called assoc_fn1");
//!     }
//!
//!     fn assoc_fn2(x: i32) -> bool {
//!         x > 0
//!     }
//! }
//!
//! // Test the generated vtable
//! #[test]
//! fn test_svtable() {
//!     let vtable = ASvtable::<Implementor>::new();
//!     
//!     // Test assoc_fn1
//!     (vtable.assoc_fn1)();  // This should print "Called assoc_fn1"
//!     
//!     // Test assoc_fn2
//!     assert!((vtable.assoc_fn2)(5));
//!     assert!(!(vtable.assoc_fn2)(-5));
//! }
//! ```
//!
//! This example demonstrates:
//! 1. Defining a trait with the `svtable` attribute
//! 2. Implementing the trait for a struct
//! 3. Creating and using the generated vtable
//! 4. Testing the vtable's functionality
//!
//! ## Generated Structure
//!
//! For the above example, the macro will generate something similar to:
//!
//! ```rust,ignore
//! pub struct ASvtable<T: A> {
//!     assoc_fn1: fn(),
//!     assoc_fn2: fn(i32) -> bool,
//!     _phantom: std::marker::PhantomData<T>,
//! }
//!
//! impl<T: A> ASvtable<T> {
//!     pub const fn new() -> Self {
//!         Self {
//!             assoc_fn1: T::assoc_fn1,
//!             assoc_fn2: T::assoc_fn2,
//!             _phantom: std::marker::PhantomData,
//!         }
//!     }
//! }
//! ```
//!
//! ## Custom Naming
//!
//! You can specify a custom name for the generated struct using the `name` attribute:
//!
//! ```rust
//! use svtable::svtable;
//!
//! #[svtable(name = "CustomVTable")]
//! pub trait B {
//!     fn method();
//! }
//! ```
//!
//! This will generate a struct named `CustomVTable<T: B>` instead of `BSvtable<T: B>`.
//!
//! ## Benefits
//!
//! - Compile-time generation of vtables
//! - Potential performance improvements over dynamic dispatch
//! - Customizable struct names for generated vtables
//! - Automatic updates to vtable when trait methods change
//!
//! ## Advanced Testing
//!
//! For more comprehensive testing, you might want to verify:
//!
//! 1. The generated struct has the expected name
//! 2. The generated struct has fields corresponding to each trait method
//! 3. The `new()` method correctly initializes the vtable
//!
//! Here's an example of more advanced testing:
//!
//! ```rust
//! use svtable::svtable;
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[svtable]
//!     trait TestTrait {
//!         fn method1();
//!         fn method2(x: i32) -> bool;
//!     }
//!
//!     struct TestImpl;
//!
//!     impl TestTrait for TestImpl {
//!         fn method1() {}
//!         fn method2(x: i32) -> bool { x > 0 }
//!     }
//!
//!     #[test]
//!     fn test_svtable_generation() {
//!         let vtable = TestTraitSvtable::<TestImpl>::new();
//!
//!         // Test that the vtable has the correct methods
//!         assert!(std::mem::size_of_val(&vtable.method1) == std::mem::size_of::<fn()>());
//!         assert!(std::mem::size_of_val(&vtable.method2) == std::mem::size_of::<fn(i32) -> bool>());
//!
//!         // Test that the methods in the vtable work correctly
//!         (vtable.method1)();
//!         assert!((vtable.method2)(5));
//!         assert!(!(vtable.method2)(-5));
//!     }
//! }
//! ```
//!
//! This test ensures that:
//! - The macro generates a struct with the expected name (`TestTraitSvtable`)
//! - The generated struct has fields corresponding to each trait method
//! - The `new()` method correctly initializes the vtable with working function pointers
//!
//! For more detailed information on usage and implementation, refer to the crate documentation.

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, ItemTrait, LitStr, Meta};

#[proc_macro_attribute]
pub fn svtable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let vis = &input.vis;

    // Parse the attribute to get the custom name if provided
    let custom_name = if !attr.is_empty() {
        parse_macro_input!(attr as syn::Meta)
            .require_list()
            .ok()
            .and_then(|list| {
                list.parse_args_with(Punctuated::<syn::Meta, Comma>::parse_terminated)
                    .ok()
            })
            .and_then(|metas| {
                metas.into_iter().find_map(|meta| {
                    if let syn::Meta::NameValue(name_value) = meta {
                        if name_value.path.is_ident("name") {
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            }) = name_value.value
                            {
                                return Some(lit_str.value());
                            }
                        }
                    }
                    None
                })
            })
    } else {
        None
    };

    let svtable_name = match custom_name {
        Some(name) => syn::Ident::new(&name, trait_name.span()),
        None => syn::Ident::new(&format!("{}Svtable", trait_name), trait_name.span()),
    };

    let svtable_fields = input.items.iter().filter_map(|item| {
        if let syn::TraitItem::Fn(assoc_fn) = item {
            let fn_name = &assoc_fn.sig.ident;
            let inputs = &assoc_fn.sig.inputs;
            let output = &assoc_fn.sig.output;
            Some(quote! {
                #fn_name: fn(#inputs) #output
            })
        } else {
            None
        }
    });

    let svtable_field_assignments = input.items.iter().filter_map(|item| {
        if let syn::TraitItem::Fn(assoc_fn) = item {
            let fn_name = &assoc_fn.sig.ident;
            Some(quote! {
                #fn_name: T::#fn_name
            })
        } else {
            None
        }
    });

    let expanded = quote! {
        #input

        #[allow(non_camel_case_types)]
        #vis struct #svtable_name<T: #trait_name> {
            #(#svtable_fields,)*
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: #trait_name> #svtable_name<T> {
            pub const fn new() -> Self {
                Self {
                    #(#svtable_field_assignments,)*
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
