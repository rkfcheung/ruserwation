use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

// Trait to verify that a method was invoked a specific number of times
pub trait MockVerify {
    // Verifies if a method was invoked the specified number of times
    fn verify_invoked(&self, method: &str, times: usize);
}

// A generic struct to capture arguments passed to mocked methods
#[derive(Default)]
pub struct ArgumentCaptor<T: Clone> {
    // Stores captured arguments in a vector inside a RefCell for interior mutability
    captured: RefCell<Vec<T>>,
}

// Represents a single captured argument value, allowing dynamic typing
#[derive(Clone)]
pub struct ArgumentValue {
    // Stores the value as a boxed trait object implementing `Any`
    value: Rc<dyn Any>,
}

// Represents a default value for mock objects when cloning
#[derive(Clone, Default)]
pub struct MockDefault;

// Tracks method invocations and captures their arguments for verification
#[derive(Default)]
pub struct InvocationTracker {
    // Tracks the number of times each method was invoked
    invoked_count: RefCell<HashMap<String, usize>>,

    // Tracks arguments captured for each method
    captors: RefCell<HashMap<String, ArgumentCaptor<ArgumentValue>>>,
}

impl<T: Clone> ArgumentCaptor<T> {
    // Captures a value by adding it to the list of captured arguments
    pub fn capture(&self, value: T) {
        self.captured.borrow_mut().push(value);
    }

    // Returns the first captured argument, if any
    pub fn first(&self) -> Option<T> {
        self.captured.borrow().first().cloned()
    }

    // Returns the last captured argument, if any
    pub fn last(&self) -> Option<T> {
        self.captured.borrow().last().cloned()
    }

    // Returns all captured arguments
    pub fn values(&self) -> Vec<T> {
        self.captured.borrow().clone()
    }
}

// Allows `ArgumentCaptor` to be sent between threads
unsafe impl<T: Clone> Send for ArgumentCaptor<T> {}

// Allows `ArgumentCaptor` to be shared between threads
unsafe impl<T: Clone> Sync for ArgumentCaptor<T> {}

impl ArgumentValue {
    // Creates a new `ArgumentValue` by boxing the given value
    pub fn new<T: Any + Clone>(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }

    // Attempts to downcast the stored value to the given type
    pub fn get<T: Any + Clone>(&self) -> Option<&T> {
        self.value.downcast_ref::<T>()
    }
}

impl Default for ArgumentValue {
    // Provides a default implementation for `ArgumentValue` using `MockDefault`
    fn default() -> Self {
        Self {
            value: Rc::new(MockDefault),
        }
    }
}

// Allows `ArgumentValue` to be sent between threads
unsafe impl Send for ArgumentValue {}

// Allows `ArgumentValue` to be shared between threads
unsafe impl Sync for ArgumentValue {}

impl InvocationTracker {
    // Increments the invocation count for the given method
    pub fn increment(&self, method: &str) {
        let mut invoked_count = self.invoked_count.borrow_mut();
        let value = invoked_count.entry(method.to_string()).or_insert(0);
        *value += 1;
    }

    // Gets the invocation count for the given method
    pub fn get(&self, method: &str) -> usize {
        *self.invoked_count.borrow().get(method).unwrap_or(&0)
    }

    // Captures the arguments for the given method
    pub fn capture<T: Clone + 'static>(&self, method: &str, arguments: T) {
        let mut captors = self.captors.borrow_mut();
        let captor = captors.entry(method.to_string()).or_default();
        captor.capture(ArgumentValue::new(arguments));
    }

    // Returns the first captured argument for the given method
    pub fn first<T: Clone + 'static>(&self, method: &str) -> Option<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .and_then(|captor| captor.first())
    }

    // Returns the last captured argument for the given method
    pub fn last<T: Clone + 'static>(&self, method: &str) -> Option<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .and_then(|captor| captor.last())
    }

    // Returns all the captured arguments for the given method
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
