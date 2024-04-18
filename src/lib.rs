use std::{cell::RefCell, collections::{HashMap, HashSet}};

use reactive::Reactive;
use uuid::Uuid;

pub mod reactive;

pub struct Reactor<'a> {
    data: RefCell<HashMap<Uuid, &'a dyn Reactive>>,
    subscriptions: RefCell<HashMap<Uuid, HashSet<Uuid>>>,
}

impl<'a> Reactor<'a> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            subscriptions: Default::default(),
        }
    }

    pub fn register(&self, data: &'a dyn Reactive) {
        if !self.data.borrow().contains_key(&data.uuid()) {
            self.subscriptions.borrow_mut()
                .insert(data.uuid(), Default::default());
            
            self.data.borrow_mut()
                .insert(data.uuid(), data);
        }
    }

    pub fn subscribe(&self, source: Uuid, sink: Uuid) {
        if self.subscriptions.borrow().contains_key(&source) {
            self.subscriptions
                .borrow_mut()
                .get_mut(&source)
                .unwrap()
                .insert(sink);
        }
    }

    pub fn broadcast_dirty(&self, source: Uuid) {
        if let Some(set) = self.subscriptions.borrow().get(&source) {
            let data = self.data.borrow();

            set.iter()
                .filter(|id| 
                    data.get(id)
                        .map(|d| d.is_clean())
                        .is_some_and(|d| d))
                .for_each(|id| {
                    data.get(id)
                        .map(|d| d.mark_dirty()); });
        }
    }
}

impl Default for Reactor<'_> {
    fn default() -> Self {
        Self { data: Default::default(), subscriptions: Default::default() }
    }
}