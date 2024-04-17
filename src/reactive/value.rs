use std::{cell::RefCell, collections::LinkedList, rc::Rc};

use uuid::Uuid;

use super::Reactive;

pub struct Value<T> {
    value: T,
    subscribers: RefCell<LinkedList<Rc<Box<dyn Reactive>>>>,
    uuid: Uuid
}

impl<T> Reactive for Value<T> {
    fn mark_dirty(&self) {
        self.subscribers
            .borrow()
            .iter()
            .filter(|s| s.is_clean())
            .for_each(|s| s.mark_dirty());
    }

    fn is_clean(&self) -> bool {
        true
    }

    fn subscribe(&self, subscriber: Rc<Box<dyn Reactive>>) {
        if self.subscribers.borrow().iter().all(|subs| subs.identity() != subscriber.identity()) {
            self.subscribers.borrow_mut().push_back(subscriber);
        }
    }

    fn identity(&self) -> Uuid {
        self.uuid
    }
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            subscribers: Default::default(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn read(&self, subscriber: Rc<Box<dyn Reactive>>) -> &T {
        &self.value
    }

    pub fn write(&mut self, value: T) {
        self.value = value;
        self.mark_dirty();
    }
}