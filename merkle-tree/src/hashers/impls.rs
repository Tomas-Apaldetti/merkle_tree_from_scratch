use crate::encoding::Digestable;

use super::CryptoHash;

impl CryptoHash{
    pub(crate) fn new_32bit_word(data: &[u32], expected_bits: u16) -> Result<Self, ()>{
        let mut new = Vec::with_capacity(
            (expected_bits >> 3) as usize
        );
        for x in data{
            new.extend(x.to_be_bytes());
        }

        if new.len() * 8 != expected_bits as usize {
            return Err(());
        }
        
        Ok(Self { data: new })
    }
}

impl Digestable for CryptoHash{
    fn bits(&self) -> &[u8] {
        &self.data
    }
}