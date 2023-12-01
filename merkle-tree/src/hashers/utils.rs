use crate::utils::num_traits::{OverflowingAdd, Zero};

pub(super) fn mod_sum<T: OverflowingAdd + Zero>(numbers: &[T]) -> T {
    if numbers.len() == 0 {
        return T::zero();
    }
    if numbers.len() == 1 {
        return numbers[0];
    }

    let (result, _) = numbers[0].overflow_add(mod_sum(&numbers[1..]));
    result
}
