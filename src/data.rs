use std::{cell::{Cell, Ref, RefCell}, collections::LinkedList, fmt::{Debug, Display}};

use uuid::Uuid;

/// Represents data that must have Reactive behaviour. Data can be both a `Value` or an `Expr`.
/// 
/// If you imagine the dependency digraph of reactive data, `Value` are sources, it means, they have no dependencies at all. 
/// On the other hand, `Expr` depend on some `Value`s and can depend on some `Expr`. So their dependency state need to be checked whenever a read is done.
/// Also, notice that `Value` are assignable, while `Expr` aren't
/// 
/// Data is said _clean_ when its value doesn't require recalculation, it means the data it depends on hasn't suffered any changes since the last time it got calculated.
/// Whenever a `Value` is reassigned, all its subscribers (data that depend on it) are marked as _dirty_, and this is propagated to all their subscribers too.
/// `Expr` marked as dirty are recalculated when a `read` is done.
pub enum ReactiveData<T> {
    Value {
        value: RefCell<T>,
        subscribers: RefCell<LinkedList<Box<dyn Reactive>>>,
        uuid: Uuid
    },
    Expr {
        value: RefCell<Option<T>>,
        expr: Box<dyn Fn(Box<Self>) -> T>,
        subscribers: RefCell<LinkedList<Box<dyn Reactive>>>,
        clean: Cell<bool>,
        uuid: Uuid
    }
}

pub trait Reactive {
    fn is_clean(&self) -> bool;

    fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    fn mark_dirty(&mut self);

    fn identity(&self) -> &Uuid;
}

impl<T> Reactive for ReactiveData<T>{
    fn is_clean(&self) -> bool {
        if let ReactiveData::Expr { clean, value, .. } = self {
            clean.get() && value.borrow().is_some()
        } else {
            true   
        }
    }

    fn mark_dirty(&mut self) {
        let subscribers = match self {
            ReactiveData::Value { subscribers, .. } => subscribers,
            ReactiveData::Expr { subscribers, clean, .. } => {
                if clean.get() {
                    clean.set(false);
                }
                
                subscribers
            },
        };

        subscribers
            .borrow_mut()
            .iter_mut()
            .filter(|d| d.is_dirty())
            .for_each(|d| d.mark_dirty());
    }

    fn identity(&self) -> &Uuid {
        match self {
            ReactiveData::Value { uuid, .. } => uuid,
            ReactiveData::Expr { uuid, .. } => uuid,
        }
    }
}

impl<T> ReactiveData<T> {
    pub fn new_value(value: T) -> Self {
        Self::Value { 
            value: RefCell::new(value), 
            subscribers: Default::default() ,
            uuid: Uuid::new_v4()
        }
    }

    pub fn new_expr<F: Fn(Box<Self>) -> T + 'static>(expr: F) -> Self {
        Self::Expr { 
            value: RefCell::new(None), 
            expr: Box::new(expr), 
            subscribers: Default::default(), 
            clean: Cell::new(false),
            uuid: Uuid::new_v4()
        }
    }

    pub fn read(&self, subscriber: Box<dyn Reactive>) -> Ref<T> {
        match self {
            ReactiveData::Value { value, subscribers, ..} => { 
                if !subscribers.borrow().iter().any(|sub| sub.identity() == self.identity()) {
                    subscribers.borrow_mut().push_back(subscriber);
                }

                value.borrow()
            },
            ReactiveData::Expr { value, expr, subscribers, clean , ..} => {
                if self.is_dirty() {
                    *value.borrow_mut() = Some((expr)(Box::new(self)));
                    clean.set(true);

                }

                if !subscribers.borrow().iter().any(|sub| sub.identity() == self.identity()) {
                    subscribers.borrow_mut().push_back(subscriber);
                }

                Ref::map(value.borrow(), |optref| optref.as_ref().unwrap())
            },
        }
    }

    pub fn write(&self, value: T) {
        if let Self::Value { value: inner_value, subscribers, .. } = self {
            *inner_value.borrow_mut() = value;

            subscribers
                .borrow_mut()
                .iter_mut()
                .filter(|s| s.is_clean())
                .for_each(|s| s.mark_dirty());
        }
    }
}

impl<T: Debug> Debug for ReactiveData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value { value, .. } => write!(f, "Reactive::Value ({:?})", value.borrow()),
            Self::Expr { value, clean, .. } => write!(f, "Reactive::Expr ({:?}){}", value.borrow(), if clean.get() {""} else {"*"}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReactiveData;

    #[test]
    fn simple_value() {
        let v = ReactiveData::new_value(40);
        dbg!(&v);
        v.write(89);
        dbg!(&v);
        assert!(*v.read(Box::new(ReactiveData::new_value(1))) == 89);
    }

    #[test]
    fn reactive_sum() {
        let a = ReactiveData::new_value(1);
        let b = ReactiveData::new_value(1);
        let sum = ReactiveData::new_expr(|r| *a.read(r) + *b.read(r));
    }
}