extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(MockVerify)]
pub fn mock_verify_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote::quote! {
        impl #name {
            pub fn verify_result(&self, method: &str, times: usize) {
                let actual = self.called_count.get(method);
                assert_eq!(actual, times, "Expected method '{}' to be called {} times, but it was called {} times,", method, times, actual);
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn mock_call_count(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Extract function details
    let fn_attrs = &input.attrs;
    let fn_sig = &input.sig;
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_vis = &input.vis;

    // Generate expanded function
    let expanded = quote::quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Increment call count for this function
            self.called_count.increment(stringify!(#fn_name));

            // Original function body
            #fn_body
        }
    };

    TokenStream::from(expanded)
}
