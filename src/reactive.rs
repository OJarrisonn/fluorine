use std::rc::Rc;

use uuid::Uuid;

pub mod value;
pub mod expr;

pub trait Reactive {
    fn mark_dirty(&self);
    
    fn is_clean(&self) -> bool;

    fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    fn subscribe(&self, subscriber: Rc<Box<dyn Reactive>>); 

    fn identity(&self) -> Uuid;
}