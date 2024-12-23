use std::{any::Any, cell::RefCell, collections::HashMap};

pub trait MockAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_box(&self) -> Box<dyn MockAny>;
}

pub trait MockVerify {
    fn verify_result(&self, method: &str, times: usize);
}

#[derive(Default)]
pub struct ArgumentCaptor<T: Clone> {
    captured: RefCell<Vec<T>>,
}

pub struct ArgumentValue {
    value: Box<dyn MockAny>,
}

#[derive(Default, Clone)]
pub struct MockDefault;

#[derive(Default)]
pub struct InvocationTracker {
    invoked_count: RefCell<HashMap<String, usize>>,
    captors: RefCell<HashMap<String, ArgumentCaptor<ArgumentValue>>>,
}

impl<T: Clone> ArgumentCaptor<T> {
    pub fn capture(&self, value: T) {
        self.captured.borrow_mut().push(value);
    }

    pub fn first(&self) -> Option<T> {
        self.captured.borrow().first().cloned()
    }

    pub fn last(&self) -> Option<T> {
        self.captured.borrow().last().cloned()
    }
}

unsafe impl<T: Clone> Send for ArgumentCaptor<T> {}

unsafe impl<T: Clone> Sync for ArgumentCaptor<T> {}

impl ArgumentValue {
    pub fn new<T: Any + Clone>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    pub fn get<T: Any + Clone>(&self) -> Option<&T> {
        self.value.as_ref().as_any().downcast_ref::<T>()
    }
}

impl Clone for ArgumentValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value.as_box(),
        }
    }
}

impl Default for ArgumentValue {
    fn default() -> Self {
        Self {
            value: MockDefault::default().as_box(),
        }
    }
}

unsafe impl Send for ArgumentValue {}

unsafe impl Sync for ArgumentValue {}

impl InvocationTracker {
    pub fn increment(&self, method: &str) {
        let mut invoked_count = self.invoked_count.borrow_mut();
        let value = invoked_count.entry(method.to_string()).or_insert(0);
        *value += 1;
    }

    pub fn get(&self, method: &str) -> usize {
        *self.invoked_count.borrow().get(method).unwrap_or(&0)
    }

    pub fn capture<T: Clone + 'static>(&self, method: &str, arguments: T) {
        let mut captors = self.captors.borrow_mut();
        let captor = captors
            .entry(method.to_string())
            .or_insert(ArgumentCaptor::default());
        captor.capture(ArgumentValue::new(arguments));
    }

    pub fn first<T: Clone + 'static>(&self, method: &str) -> Option<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .and_then(|captor| captor.first())
    }

    pub fn last<T: Clone + 'static>(&self, method: &str) -> Option<ArgumentValue> {
        self.captors
            .borrow()
            .get(method)
            .and_then(|captor| captor.last())
    }
}

impl<T: Any + Clone> MockAny for T {
    fn as_box(&self) -> Box<dyn MockAny> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

unsafe impl Send for InvocationTracker {}

unsafe impl Sync for InvocationTracker {}
