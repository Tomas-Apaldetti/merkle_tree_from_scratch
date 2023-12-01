pub mod overflowing_add;
pub mod zero;
pub(crate) trait OverflowingAdd where Self: Sized + Copy{
    fn overflow_add(self, rhs: Self) -> (Self, bool);
}

pub(crate) trait Zero{
    fn zero() -> Self;
}