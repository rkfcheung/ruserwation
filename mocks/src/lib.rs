use std::{any::Any, cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

/// Enum to represent the type of verification check.
#[derive(Debug)]
pub enum MockCheck {
    /// Less than or equal to
    Lte,
    /// Equal to
    Eq,
    /// Greater than or equal to
    Gte,
}

/// Trait to verify that a method was invoked a specific number of times.
pub trait MockVerify {
    fn verify_invoked(&self, method: &str, check: MockCheck, times: usize);

    /// Verifies if a method was invoked exactly the specified number of times.
    fn verify_exactly(&self, method: &str, times: usize) {
        self.verify_invoked(method, MockCheck::Eq, times);
    }

    /// Verifies if a method was invoked at least the specified number of times.
    fn verify_at_least(&self, method: &str, times: usize) {
        self.verify_invoked(method, MockCheck::Gte, times);
    }

    /// Verifies if a method was invoked at most the specified number of times.
    fn verify_at_most(&self, method: &str, times: usize) {
        self.verify_invoked(method, MockCheck::Lte, times);
    }

    /// Verifies if a method was invoked exactly once.
    fn verify_exactly_once(&self, method: &str) {
        self.verify_exactly(method, 1);
    }

    /// Verifies if a method was invoked at least once.
    fn verify_at_least_once(&self, method: &str) {
        self.verify_at_least(method, 1);
    }

    /// Verifies if a method was invoked at most once.
    fn verify_at_most_once(&self, method: &str) {
        self.verify_at_most(method, 1);
    }

    /// Verifies if a method was never invoked.
    fn verify_never(&self, method: &str) {
        self.verify_exactly(method, 0);
    }
}

/// A generic struct to capture arguments passed to mocked methods.
#[derive(Default)]
pub struct ArgumentCaptor<T: Clone> {
    /// Stores captured arguments in a vector with interior mutability.
    captured: RefCell<Vec<T>>,
}

/// Represents a single captured argument value, allowing dynamic typing.
#[derive(Clone, Debug)]
pub struct ArgumentValue {
    /// Stores the value as an `Rc` trait object implementing `Any`.
    value: Rc<dyn Any>,
}

/// Tracks method invocations and captures their arguments for verification.
#[derive(Default)]
pub struct InvocationTracker {
    /// Tracks the number of times each method was invoked.
    invoked_count: RefCell<HashMap<String, usize>>,

    /// Tracks arguments captured for each method.
    captors: RefCell<HashMap<String, ArgumentCaptor<ArgumentValue>>>,
}

/// Represents the result of a mock verification.
#[derive(Debug, Default)]
pub struct MockAnswer {
    pub passed: bool,
    pub reason: String,
}

/// Represents a default value for mock objects.
#[derive(Clone, Default)]
pub struct MockDefault;

impl<T: Clone> ArgumentCaptor<T> {
    /// Captures a value by adding it to the list of captured arguments.
    pub fn capture(&self, value: T) {
        self.captured.borrow_mut().push(value);
    }

    /// Returns the first captured argument, if any.
    pub fn first(&self) -> Option<T> {
        self.captured.borrow().first().cloned()
    }

    /// Returns the last captured argument, if any.
    pub fn last(&self) -> Option<T> {
        self.captured.borrow().last().cloned()
    }

    /// Returns all captured arguments.
    pub fn values(&self) -> Vec<T> {
        self.captured.borrow().clone()
    }
}

// Allows `ArgumentCaptor` to be sent between threads
unsafe impl<T: Clone> Send for ArgumentCaptor<T> {}

// Allows `ArgumentCaptor` to be shared between threads
unsafe impl<T: Clone> Sync for ArgumentCaptor<T> {}

impl ArgumentValue {
    /// Creates a new `ArgumentValue` by wrapping the provided value in an `Rc`.
    ///
    /// # Arguments
    /// - `value`: The value to wrap, which must implement `Any` and `Clone`.
    ///
    /// # Example
    /// ```rust
    /// let value = mocks::ArgumentValue::new(42);
    /// ```
    pub fn new<T: Any + Clone>(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }

    /// Attempts to downcast the stored value to the given type `T`.
    ///
    /// Returns `None` if the stored value's type does not match `T`.
    ///
    /// # Example
    /// ```rust
    /// let value = mocks::ArgumentValue::new(42);
    /// assert_eq!(*value.get::<i32>().unwrap(), 42);
    /// ```
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref::<T>()
    }

    /// Attempts to downcast the stored value to the given type `T`, and panics if the types don't match.
    ///
    /// # Panics
    /// Panics if the stored value's type does not match `T`.
    ///
    /// # Example
    /// ```rust
    /// let value = mocks::ArgumentValue::new(42);
    /// assert_eq!(*value.unwrap::<i32>(), 42);
    /// ```
    pub fn unwrap<T: Any>(&self) -> &T {
        let result = self.get();
        assert!(
            result.is_some(),
            "Failed to downcast `ArgumentValue` to '{}'. Ensure the types match.",
            std::any::type_name::<T>()
        );
        result.unwrap()
    }
}

impl Default for ArgumentValue {
    /// Provides a default `ArgumentValue` wrapping a placeholder `MockDefault`.
    fn default() -> Self {
        Self {
            value: Rc::new(MockDefault),
        }
    }
}

impl Deref for ArgumentValue {
    type Target = dyn Any;

    /// Dereferences the wrapped value as `dyn Any`.
    fn deref(&self) -> &Self::Target {
        self.value.as_ref()
    }
}

// Allows `ArgumentValue` to be sent between threads
unsafe impl Send for ArgumentValue {}

// Allows `ArgumentValue` to be shared between threads
unsafe impl Sync for ArgumentValue {}

impl InvocationTracker {
    /// Increments the invocation count for the given method.
    pub fn increment(&self, method: &str) {
        let mut invoked_count = self.invoked_count.borrow_mut();
        let value = invoked_count.entry(method.to_string()).or_insert(0);
        *value += 1;
    }

    /// Gets the invocation count for the given method.
    pub fn get(&self, method: &str) -> usize {
        *self.invoked_count.borrow().get(method).unwrap_or(&0)
    }

    /// Verifies the number of times a method was invoked based on the given check.
    pub fn verify_invoked(&self, method: &str, check: MockCheck, times: usize) -> MockAnswer {
        let actual = self.get(method);

        // Perform the comparison based on the provided check.
        let passed = match check {
            MockCheck::Lte => actual <= times,
            MockCheck::Eq => actual == times,
            MockCheck::Gte => actual >= times,
        };

        // Construct the failure reason.
        let reason = if passed {
            format!("Verification passed: method '{method}' was called {actual} times as expected.")
        } else {
            format!(
                "Expected method '{method}' to be called {:?} {times} times, but it was called {actual} times.",
                check
            )
        };

        // Return the verification result.
        MockAnswer { passed, reason }
    }

    /// Captures the arguments for the given method.
    pub fn capture<T: Clone + 'static>(&self, method: &str, arguments: T) {
        let mut captors = self.captors.borrow_mut();
        let captor = captors.entry(method.to_string()).or_default();
        captor.capture(ArgumentValue::new(arguments));
    }

    /// Returns the first captured argument for the given method.
    pub fn first(&self, method: &str) -> Option<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .and_then(|captor| captor.first())
    }

    /// Returns the last captured argument for the given method.
    pub fn last(&self, method: &str) -> Option<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .and_then(|captor| captor.last())
    }

    /// Returns all captured arguments for the given method.
    pub fn values(&self, method: &str) -> Vec<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .map(|captor| captor.values())
            .unwrap_or_default()
    }
}

// Allows `InvocationTracker` to be sent between threads
unsafe impl Send for InvocationTracker {}

// Allows `InvocationTracker` to be shared between threads
unsafe impl Sync for InvocationTracker {}
