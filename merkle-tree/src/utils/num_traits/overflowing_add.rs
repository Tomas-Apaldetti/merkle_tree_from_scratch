use super::OverflowingAdd;

impl OverflowingAdd for u32{
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingAdd for u64{
    fn overflow_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}