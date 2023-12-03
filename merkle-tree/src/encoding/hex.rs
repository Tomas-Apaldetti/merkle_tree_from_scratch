use std::marker::PhantomData;

use super::Digester;

const HEX_MAPPING: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];

struct Hex{
    non_instance: PhantomData<bool>,
}

impl Digester for Hex{
    fn digest(bits: &[u8]) -> String {
        let mut s = String::with_capacity(bits.len() * 2);

        for bit in bits{
            s.push(HEX_MAPPING[(bit & 0xF0) as usize]);
            s.push(HEX_MAPPING[(bit & 0x0F) as usize]);
        }

        s
    }
}