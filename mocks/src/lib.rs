use std::{cell::RefCell, collections::HashMap};

pub trait MockVerify {
    fn verify_result(&self, method: &str, times: usize);
}

#[derive(Default)]
pub struct CalledCount {
    counter: RefCell<HashMap<String, usize>>,
}

impl CalledCount {
    pub fn increment(&self, method: &str) {
        let mut count_map = self.counter.borrow_mut();
        let value = count_map.entry(method.to_string()).or_insert(0);
        *value += 1;
    }

    pub fn get(&self, method: &str) -> usize {
        *self.counter.borrow().get(method).unwrap_or(&0)
    }
}

unsafe impl Send for CalledCount {}

unsafe impl Sync for CalledCount {}
