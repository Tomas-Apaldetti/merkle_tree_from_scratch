use std::marker::PhantomData;

use super::Digester;

const HEX_MAPPING: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];

pub struct Hex{
    non_instance: PhantomData<bool>,
}

impl Digester for Hex{
    fn digest(bits: &[u8]) -> String {
        let mut s = String::with_capacity(bits.len() * 2);

        for bit in bits{
            s.push(HEX_MAPPING[((bit & 0xF0) >> 4) as usize]);
            s.push(HEX_MAPPING[(bit & 0x0F) as usize]);
        }

        s
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn hex_digester(){
        let pairs = [
            (b"This is a test".as_slice(), "5468697320697320612074657374"),
            (b"This is a test with a longer string that has pure ASCII characters".as_slice(), "546869732069732061207465737420776974682061206C6F6E67657220737472696E6720746861742068617320707572652041534349492063686172616374657273"),
            (b"".as_slice(), ""),
            (b"\xF0\x9F\x98\x81".as_slice(), "F09F9881")
        ];

        for pair in pairs{
            assert_eq!(Hex::digest(pair.0), pair.1);
        }
    }
}