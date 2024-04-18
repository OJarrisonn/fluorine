
use std::cell::{Ref, RefCell};

use uuid::Uuid;

use crate::Reactor;

use super::Reactive;

pub struct ReactiveValue<'a, T> {
    value: RefCell<T>,
    uuid: Uuid,
    reactor: &'a Reactor<'a>
}

impl<T> Reactive for ReactiveValue<'_, T> {
    #[inline(always)]
    fn uuid(&self) -> Uuid {
        self.uuid
    }
    
    #[inline(always)]
    fn is_clean(&self) -> bool {
        true
    }
    
    fn mark_dirty(&self) {
        panic!("Value {} was marked dirty, which is impossible since a value depends on no one", self.uuid)
    }

    
}

impl<'a, T: 'a> ReactiveValue<'a, T> {
    pub fn new(value: T, reactor: &'a Reactor<'a>) -> Self {
        let val = Self {
            value: value.into(),
            uuid: Uuid::new_v4(),
            reactor,
        };

        val
    }

    pub fn read(&self, uuid: Uuid) -> Ref<T> {
        self.reactor.subscribe(self.uuid, uuid);

        self.seek()
    }

    #[inline(always)]
    pub fn seek(&self) -> Ref<T> {
        self.value.borrow()
    }

    #[inline(always)]
    pub fn write(&self, value: T) {
        self.reactor.broadcast_dirty(self.uuid);

        *self.value.borrow_mut() = value;
    } 
}

#[cfg(test)]
mod tests {
    use crate::Reactor;

    use super::ReactiveValue;

    #[test]
    fn create_value() {
        let r = Reactor::new();
        let a = ReactiveValue::new(89, &r);
        let b = ReactiveValue::new("Hello World", &r);
        r.register(&a);
        r.register(&b);
        dbg!(a.seek());
    }
}