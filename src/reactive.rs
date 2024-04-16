pub trait Reactive {
    fn mark_dirty(&self);
    
    fn is_clean(&self) -> bool;

    fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    fn subscribe(&self, subscriber: Box<dyn Reactive>); 
}