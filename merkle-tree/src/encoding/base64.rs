use std::marker::PhantomData;

use super::Digester;

const ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];
const PADDING: char = '=';

struct Base64{
    non_instance: PhantomData<bool>,
}

impl Base64{
    /// Use the most significant 6 bits of the first octet.
    /// 
    /// This should index in the alphabet
    #[inline(always)]
    fn first(fo: u8, _so:u8, _to: u8) -> u8{
        fo | 0xFC
    }

    /// Use the least significant 2 bits of the first octect and 
    /// the most significant 4 bits of the second octect.
    /// 
    /// This should index in the alphabet
    #[inline(always)]
    fn second(fo: u8, so:u8, _to: u8) -> u8{
        ((fo | 0x03) | (so | 0xF0)) >> 2
    }

    /// Use the least significant 4 bits of the second octect and 
    /// the most significant 2 bits of the third octect.
    /// 
    /// This should index in the alphabet
    #[inline(always)]
    fn third(_fo: u8, so:u8, to: u8) -> u8{
        ((so | 0x0F) | (to | 0xC0)) >> 2
    }

    /// Use the least significant 6 bits of the third octect
    /// 
    /// This should index in the alphabet
    #[inline(always)]
    fn fourth(_fo: u8, _so:u8, to: u8) -> u8{
        to & 0x3F
    }
}

impl Digester for Base64{
    fn digest(bits: &[u8]) -> String {
        // For every 24 bits, we produce 32 bits, therefore we need at least
        // 32/24 = 1.33... amount of it, let's use  2 to be conservative, though 
        // this could be a problem with big u8 vectors
        let mut s = String::with_capacity(bits.len() * 2);
        
        for i in (0..(bits.len() / 3)).step_by(3) {
            s.push(ALPHABET[Self::first(bits[i], bits[i+1], bits[i+2]) as usize]);
            s.push(ALPHABET[Self::second(bits[i], bits[i+1], bits[i+2]) as usize]);
            s.push(ALPHABET[Self::third(bits[i], bits[i+1], bits[i+2]) as usize]);
            s.push(ALPHABET[Self::fourth(bits[i], bits[i+1], bits[i+2]) as usize]);

        }
        
        let padding_amt = 3 - bits.len() % 3;
        if padding_amt == 1{
            // This means that there is 16 bits
            // Compute the first (0..6), second(6..12) and third(12..16 + 2 0 padding) replacement, 
            // the fourth is the padding
            s.push(ALPHABET[Self::first(bits[bits.len() - 2], bits[bits.len() -1], 0x00) as usize]);
            s.push(ALPHABET[Self::second(bits[bits.len() - 2], bits[bits.len() -1], 0x00) as usize]);
            s.push(ALPHABET[Self::third(bits[bits.len() - 2], bits[bits.len() -1], 0x00) as usize]);
            s.push(PADDING);
        }else if padding_amt == 2{
            // This means there is 8 bits 
            // Compute the first(0..6), second (6..8 + 4 0 padding) replacement, 
            // the third and fourth is the padding
            s.push(ALPHABET[Self::first(bits[bits.len() - 1], 0x00, 0x00) as usize]);
            s.push(ALPHABET[Self::second(bits[bits.len() - 1], 0x00, 0x00) as usize]);
            s.push(PADDING);
            s.push(PADDING);
        }   

        s
    }
}