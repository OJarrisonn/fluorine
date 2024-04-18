use std::cell::{Cell, Ref, RefCell};

use uuid::Uuid;

use crate::Reactor;

use super::Reactive;

pub struct ReactiveExpr<'a, F, T>
where F: Fn(Uuid) -> T {
    value: RefCell<Option<T>>,
    expr: F,
    uuid: Uuid,
    reactor: &'a Reactor<'a>,
    clean: Cell<bool>
}

impl<'a, F: Fn(Uuid) -> T, T> Reactive for ReactiveExpr<'a, F, T> {
    #[inline(always)]
    fn uuid(&self) -> Uuid {
        self.uuid
    }
    
    #[inline(always)]
    fn is_clean(&self) -> bool {
        self.clean.get() && self.value.borrow().is_some()
    }
    
    fn mark_dirty(&self) {
        self.clean.set(false);

        self.reactor.broadcast_dirty(self.uuid);
    }
}

impl<'a, F: Fn(Uuid) -> T, T> ReactiveExpr<'a, F, T> {
    pub fn new(expr: F, reactor: &'a Reactor<'a>) -> Self {
        Self {
            value: Default::default(),
            expr,
            uuid: Uuid::new_v4(),
            reactor,
            clean: Cell::new(false)
        }
    }

    fn recompute(&self) {
        *self.value.borrow_mut() = Some((self.expr)(self.uuid));
        self.clean.set(true);
    }

    pub fn read(&self, uuid: Uuid) -> Ref<T> {
        self.reactor.subscribe(self.uuid, uuid);

        self.seek()
    }

    pub fn seek(&self) -> Ref<T> {
        if self.is_dirty() {
            self.recompute()
        }

        Ref::map(self.value.borrow(), |optref| optref.as_ref().expect("`Value` should be some"))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::{reactive::value::ReactiveValue, Reactor};

    use super::ReactiveExpr;

    #[test]
    fn reactive_sum() {
        let react = Reactor::new();

        let a = ReactiveValue::new(1, &react);
        let b = ReactiveValue::new(1, &react);
        react.register(&a);
        react.register(&b);

        let sum = ReactiveExpr::new(|this| *a.read(this) + *b.read(this), &react);
        react.register(&sum);
        
        dbg!(sum.seek());
        a.write(2);
        dbg!(sum.seek());
    }
}