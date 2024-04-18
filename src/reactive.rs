use uuid::Uuid;

pub mod value;
pub mod expr;
pub mod statement;

pub trait Reactive {
    fn uuid(&self) -> Uuid;

    fn is_clean(&self) -> bool;

    #[inline(always)]
    fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    fn mark_dirty(&self);
}