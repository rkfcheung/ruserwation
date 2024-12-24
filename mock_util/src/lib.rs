extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(MockVerify)]
pub fn mock_verify_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote::quote! {
        impl mocks::MockVerify for #name {
            fn verify_exactly(&self, method: &str, times: usize) {
                let answer = self.invocation.verify_exactly(method, times);
                assert!(answer.passed, "{}", answer.reason);
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn mock_invoked(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Extract function details
    let fn_attrs = &input.attrs;
    let fn_sig = &input.sig;
    let fn_name = &fn_sig.ident;
    let fn_body = &input.block;
    let fn_vis = &input.vis;

    // Generate the expanded function
    let expanded = quote::quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Increment call count for this function
            self.invocation.increment(stringify!(#fn_name));

            // Original function body
            #fn_body
        }
    };

    TokenStream::from(expanded)
}
