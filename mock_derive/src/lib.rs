extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, FnArg, Ident, ItemFn,
    Type,
};

/// Derives the `MockVerify` trait for a given struct.
///
/// This procedural macro automatically implements the `MockVerify` trait
/// for a type, enabling verification of method invocations.
///
/// Example:
/// ```rust
/// use mock_derive::MockVerify;
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
    expanded.into()
}

/// Marks a function as "mockable" by tracking its invocations.
///
/// This procedural macro wraps a function to automatically increment
/// the invocation count for tracking and verification purposes.
///
/// Example:
/// ```rust
/// use mock_derive::mock_invoked;
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

    // Generate the increment statement
    let increment_statement = prepare_increment_statement(fn_name);

    // Generate the expanded function with invocation tracking
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Track invocation count for this function
            #increment_statement

            // Execute the original function body
            #fn_body
        }
    };

    // Convert the generated code back into a token stream
    expanded.into()
}

/// Marks a function as "mockable" by capturing its arguments and tracking invocations.
///
/// This procedural macro automatically wraps a function to capture its arguments and increment
/// the invocation count for tracking and verification purposes.
///
/// It is particularly useful for mocking and verifying interactions with functions in unit tests.
///
/// Example:
/// ```rust
/// use mock_derive::mock_captured_arguments;
/// use std::sync::Arc;
///
/// // Mock structure for testing
/// struct MyMock {
///     invocation: mocks::InvocationTracker,
/// }
///
/// impl MyMock {
///     #[mock_captured_arguments]
///     fn add(&self, a: i32, b: i32) -> i32 {
///         a + b // Original function body
///     }
///
///     #[mock_captured_arguments]
///     fn greet(&self, message: &str) {
///         println!("Greeting: {}", message);
///     }
///
///     #[mock_captured_arguments]
///     fn hi(&self) {
///         println!("Hi!");
///     }
/// }
///
/// // Example usage in a test
/// fn main() {
///     // Create a mock instance
///     let mock = MyMock {
///         invocation: mocks::InvocationTracker::default(),
///     };
///
///     // Call the `add` function and verify the captured arguments
///     let result = mock.add(10, 20);
///     assert_eq!(result, 30); // Verify the return value
///     let captured_add_args = mock.invocation.values("add");
///     assert_eq!(captured_add_args.len(), 1); // Ensure the function was invoked once
///     let args = captured_add_args[0].get::<(i32, i32)>().unwrap();
///     assert_eq!(*args, (10, 20)); // Verify the captured arguments
///
///     // Call the `greet` function and verify the captured arguments
///     mock.greet("Hello, world!");
///     let captured_greet_args = mock.invocation.values("greet");
///     assert_eq!(captured_greet_args.len(), 1);
///     let message = captured_greet_args[0].get::<String>().unwrap();
///     assert_eq!(message, "Hello, world!");
///
///     // Call the `hi` function (no arguments, so nothing should be captured)
///     mock.hi();
///     let captured_hi_args = mock.invocation.values("hi");
///     assert_eq!(captured_hi_args.len(), 0); // No arguments to capture
/// }
/// ```
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

    // Generate the capture statement
    let capture_statement = prepare_capture_statement(fn_name, fn_args);

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

// Generate the increment statement
fn prepare_increment_statement(fn_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        self.invocation.increment(stringify!(#fn_name));
    }
}

// Generate the capture statement
fn prepare_capture_statement(
    fn_name: &Ident,
    fn_args: &Punctuated<FnArg, Comma>,
) -> proc_macro2::TokenStream {
    // Process arguments
    let capture_args = process_arguments(fn_args);

    if capture_args.is_empty() {
        quote! {}
    } else {
        quote! {
            self.invocation.capture(stringify!(#fn_name), (#(#capture_args),*));
        }
    }
}

// Processes function arguments and generates a list of tokens for capturing them.
fn process_arguments(fn_args: &Punctuated<FnArg, Comma>) -> Vec<proc_macro2::TokenStream> {
    let mut capture_args = vec![];
    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            // Extract argument name
            let arg_name = &*pat_type.pat;

            // Check if argument type is &str
            if let Type::Reference(ref_type) = &*pat_type.ty {
                if let Type::Path(type_path) = &*ref_type.elem {
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

    capture_args
}
