extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
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
    let expanded = quote! {
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
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Track invocation count for this function
            self.invocation.increment(stringify!(#fn_name));

            // Execute the original function body
            #fn_body
        }
    };

    // Convert the generated code back into a token stream
    expanded.into()
}

// This is the procedural macro function that will be used to capture the method arguments.
#[proc_macro_attribute]
pub fn mock_captured_arguments(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Extract function components
    let fn_attrs = &input.attrs; // Function attributes
    let fn_sig = &input.sig; // Function signature
    let fn_name = &fn_sig.ident; // Function name
    let fn_args = &fn_sig.inputs; // Function arguments
    let fn_body = &input.block; // Function body
    let fn_vis = &input.vis; // Visibility (e.g., pub)

    // Process arguments
    let mut capture_args = vec![];
    for arg in fn_args {
        if let syn::FnArg::Typed(pat_type) = arg {
            // Extract argument name
            let arg_name = &*pat_type.pat;

            // Check if argument type is &str
            if let syn::Type::Reference(ref_type) = &*pat_type.ty {
                if let syn::Type::Path(type_path) = &*ref_type.elem {
                    if type_path.path.is_ident("str") {
                        // Convert `&str` to `.to_string()`
                        capture_args.push(quote! {
                            #arg_name.to_string()
                        });
                        continue;
                    }
                }
            }

            // For all other types, pass the argument as-is
            capture_args.push(quote! {
                #arg_name
            });
        }
    }

    // Generate the capture statement
    let capture_statement = if !capture_args.is_empty() {
        quote! {
            self.invocation.capture(stringify!(#fn_name), (#(#capture_args),*));
        }
    } else {
        quote! {}
    };

    // Generate the expanded function
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #capture_statement

            #fn_body
        }
    };

    // Convert the generated code back into a token stream
    expanded.into()
}
