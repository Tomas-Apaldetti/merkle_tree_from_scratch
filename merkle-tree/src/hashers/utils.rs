use crate::utils::num_traits::{OverflowingAdd, Zero, Number, BitLength};

#[inline(always)]
pub(crate) fn rotate_right<T: Number + BitLength>(x: T, n: T) -> T {
    let max = T::bit_length();
    (x >> n) | (x << (max - n))
}

#[inline(always)]
pub(crate) fn shift_right<T: Number>(x:T, n: T) -> T {
    x >> n
}

pub(crate) fn mod_sum<T: OverflowingAdd + Zero>(numbers: &[T]) -> T {
    if numbers.len() == 0 {
        return T::zero();
    }
    if numbers.len() == 1 {
        return numbers[0];
    }
    
    let (result, _) = numbers[0].overflow_add(mod_sum(&numbers[1..]));
    result
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn mod_32bit_test(){
        assert_eq!(mod_sum(&[u32::MAX, 1u32]), 0);
        assert_eq!(mod_sum(&[u32::MAX, 0u32]), u32::MAX);
        assert_eq!(mod_sum(&[u32::MAX, 0u32, 1u32]), 0);
        assert_eq!(mod_sum(&[u32::MAX, 1u32, 1u32, 1u32]), 2);
    }
}