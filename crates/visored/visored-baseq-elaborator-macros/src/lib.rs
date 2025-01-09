use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn stashes(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Extract field names from struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect::<Vec<_>>(),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the implementation
    let field_calls = fields.iter().map(|field| {
        quote! {
            self.#field.add_hypothesis(hypothesis_record, hypothesis_entry, db);
        }
    });

    // Get the generics from the input struct
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        // Keep the original struct definition
        #input

        // Generate the implementation
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn _add_hypothesis(&mut self,
                hypothesis_record: VdBsqHypothesisStackRecord<'sess>,
                hypothesis_entry: &VdBsqHypothesisEntry<'sess>,
                db: &'sess FloaterDb,
            ) {
                #(#field_calls)*
            }
        }
    };

    expanded.into()
}
