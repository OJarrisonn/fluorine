use std::{borrow::Borrow, cell::{Cell, RefCell}, collections::LinkedList, rc::Rc};

use uuid::Uuid;

use super::Reactive;

pub struct Expr<F, T> 
where F: Fn(Rc<Box<dyn Reactive>>) -> T 
{
    value: Option<T>,
    expr: F,
    clean: Cell<bool>,
    subscribers: RefCell<LinkedList<Rc<Box<dyn Reactive>>>>,
    uuid: Uuid
}

impl<F: Fn(Rc<Box<dyn Reactive>>) -> T, T> Reactive for Expr<F, T> {
    fn mark_dirty(&self) {
        self.clean.set(false);

        self.subscribers
            .borrow()
            .iter()
            .filter(|s| s.is_clean())
            .for_each(|s| s.mark_dirty());
    }

    fn is_clean(&self) -> bool {
        self.clean.get()
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

impl<F: Fn(Rc<Box<dyn Reactive>>) -> T, T> Expr<F, T> {
    pub fn new(expr: F) -> Self {
        Self {
            value: None,
            expr,
            clean: Cell::new(false),
            subscribers: Default::default(),
            uuid: Uuid::new_v4(),
        }
    }

    fn recompute(&mut self) {
        self.value = Some((self.expr)(Rc::new(Box::new(self as &dyn Reactive))));
    }

    pub fn read(&mut self, subscriber: Rc<Box<dyn Reactive>>) -> &T {
        if self.is_dirty() {

        }

        self.subscribe(subscriber);

        self.value.as_ref().expect("Value should be `Some`")
    }
}