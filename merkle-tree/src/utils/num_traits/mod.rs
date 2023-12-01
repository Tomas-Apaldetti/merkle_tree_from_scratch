pub mod overflowing_add;
pub mod zero;
pub mod one;
pub mod max;
pub(crate) trait OverflowingAdd
where
    Self: Sized + Copy + std::ops::Add<Self, Output = Self> + Eq + Max,
{
    fn overflow_add(self, rhs: Self) -> (Self, bool);
}

pub(crate) trait Zero {
    fn zero() -> Self;
}

pub(crate) trait One {
    fn one() -> Self;
}

pub(crate) trait Max{
    fn max() -> Self;
}
