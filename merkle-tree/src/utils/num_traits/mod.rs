pub mod overflowing_add;
pub mod zero;
pub mod one;
pub mod max;
pub mod bit_length;

pub(crate) trait Number
where 
    Self: Sized + Copy + Eq + PartialEq + PartialOrd + 
    std::ops::Add<Self, Output = Self> +
    std::ops::Sub<Self, Output = Self> +
    std::ops::BitAnd<Self, Output = Self> +
    std::ops::BitOr<Self, Output = Self> +
    std::ops::BitXor<Self, Output = Self> +
    std::ops::Shl<Self, Output = Self> +
    std::ops::Shr<Self, Output = Self> 
{}

impl Number for u32{}
impl Number for u64{}
pub(crate) trait OverflowingAdd
where
    Self: Number + Max,
{
    fn overflow_add(self, rhs: Self) -> (Self, bool);
}

pub(crate) trait BitLength{
    fn bit_length()->Self;
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
