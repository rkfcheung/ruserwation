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
/// // Create a mock instance
/// let mock = MyMock {
///     invocation: mocks::InvocationTracker::default(),
/// };
///
/// // Call the `add` function and verify the captured arguments
/// let result = mock.add(10, 20);
/// assert_eq!(result, 30); // Verify the return value
/// let captured_add_args = mock.invocation.values("add");
/// assert_eq!(captured_add_args.len(), 1); // Ensure the function was invoked once
/// let args = captured_add_args[0].get::<(i32, i32)>().unwrap();
/// assert_eq!(*args, (10, 20)); // Verify the captured arguments
///
/// // Call the `greet` function and verify the captured arguments
/// mock.greet("Hello, world!");
/// let captured_greet_args = mock.invocation.values("greet");
/// assert_eq!(captured_greet_args.len(), 1);
/// let message = captured_greet_args[0].get::<String>().unwrap();
/// assert_eq!(message, "Hello, world!");
///
/// // Call the `hi` function (no arguments, so nothing should be captured)
/// mock.hi();
/// let captured_hi_args = mock.invocation.values("hi");
/// assert_eq!(captured_hi_args.len(), 0); // No arguments to capture
/// ```
#[proc_macro_attribute]
pub fn mock_captured_arguments(_attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_mock_track("Capture", item)
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
    impl_mock_track("Increment", item)
}

/// A procedural macro that tracks method invocations by incrementing counters and optionally capturing arguments.
///
/// This macro can be configured with the following attributes:
/// - `"Increment"`: Tracks the number of times the function is invoked. This is the default behavior.
/// - `"Capture"`: Captures the arguments passed to the function for later verification. This also implies tracking invocation count.
///
/// Example usage:
/// ```rust
/// use mock_derive::mock_track;
///
/// struct MyMock {
///     invocation: mocks::InvocationTracker,
/// }
///
/// impl MyMock {
///     #[mock_track("Increment")]
///     pub fn my_increment_only_function(&self) {
///         // Function body
///     }
///
///     #[mock_track("Capture")]
///     pub fn my_capture_function(&self, arg1: i32, arg2: String) {
///         // Function body
///     }
/// }
/// ```
///
/// For testing purposes, you can verify the number of invocations and captured arguments using the `InvocationTracker`.
#[proc_macro_attribute]
pub fn mock_track(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Delegate the implementation logic
    impl_mock_track(attr.to_string().trim(), item)
}

fn impl_mock_track(attr: &str, item: TokenStream) -> TokenStream {
    let capture = attr.contains("Capture"); // Check if "Capture" is specified
    let increment = attr.contains("Increment") || !capture; // Default to "Increment" if unspecified
    let input = parse_macro_input!(item as ItemFn);

    // Extract function components
    let fn_attrs = &input.attrs; // Function attributes
    let fn_sig = &input.sig; // Function signature
    let fn_name = &fn_sig.ident; // Function name
    let fn_args = &fn_sig.inputs; // Function arguments
    let fn_body = &input.block; // Function body
    let fn_vis = &input.vis; // Visibility (e.g., pub)

    // Generate the capture statement (if "Capture" is specified)
    let capture_statement = if capture {
        prepare_capture_statement(fn_name, fn_args)
    } else {
        quote! {}
    };

    // Generate the increment statement (if "Increment" is specified)
    let increment_statement = if increment {
        prepare_increment_statement(fn_name)
    } else {
        quote! {}
    };

    // Generate the expanded function
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Track invocation count
            #increment_statement
            // Capture argument values
            #capture_statement

            // Execute the original function body
            #fn_body
        }
    };

    // Convert the generated code back into a token stream
    expanded.into()
}

// Generates code to capture function arguments.
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

// Generates code to increment the invocation counter.
fn prepare_increment_statement(fn_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        self.invocation.increment(stringify!(#fn_name));
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
