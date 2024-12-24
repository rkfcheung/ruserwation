extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

/// Derives the `MockVerify` trait for a given struct.
///
/// This procedural macro automatically implements the `MockVerify` trait
/// for a type, enabling verification of method invocations.
///
/// Example:
/// ```rust
/// use mock_util::MockVerify;
///
/// #[derive(MockVerify)]
/// struct MyMock {
///     invocation: mocks::InvocationTracker,
/// }
/// ```
#[proc_macro_derive(MockVerify)]
pub fn mock_verify_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate the implementation of the `MockVerify` trait
    let expanded = quote::quote! {
        impl mocks::MockVerify for #name {
            /// Verifies the number of times a method was invoked.
            fn verify_invoked(&self, method: &str, check: mocks::MockCheck, times: usize) {
                let answer = self.invocation.verify_invoked(method, check, times);
                // Assert the verification result and provide a detailed reason on failure
                assert!(answer.passed, "{}", answer.reason);
            }
        }
    };

    // Convert the generated code back into a token stream
    TokenStream::from(expanded)
}

/// Marks a function as "mockable" by tracking its invocations.
///
/// This procedural macro wraps a function to automatically increment
/// the invocation count for tracking and verification purposes.
///
/// Example:
/// ```rust
/// use mock_util::mock_invoked;
///
/// struct MyMock {
///     invocation: mocks::InvocationTracker,
/// }
///
/// impl MyMock {
///     #[mock_invoked]
///     fn my_function(&self, arg: i32) {
///         // Original body of the function
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn mock_invoked(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let input = parse_macro_input!(item as ItemFn);

    // Extract function details
    let fn_attrs = &input.attrs; // Function attributes (e.g., #[inline])
    let fn_sig = &input.sig; // Function signature
    let fn_name = &fn_sig.ident; // Function name
    let fn_body = &input.block; // Function body
    let fn_vis = &input.vis; // Function visibility (e.g., `pub`)

    // Generate the expanded function with invocation tracking
    let expanded = quote::quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Track invocation count for this function
            self.invocation.increment(stringify!(#fn_name));

            // Execute the original function body
            #fn_body
        }
    };

    // Convert the generated code back into a token stream
    TokenStream::from(expanded)
}
