use super::BitLength;

impl BitLength for u32 {
    fn bit_length()->Self {
        32
    }
}

impl BitLength for u64 {
    fn bit_length() -> Self {
        64
    }
}
