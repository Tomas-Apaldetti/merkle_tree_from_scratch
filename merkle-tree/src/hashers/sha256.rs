use std::{cmp::min, marker::PhantomData};

use super::{utils::{mod_sum, rotate_right, shift_right}, CryptoHash, CryptoHasher};

const CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const E: usize = 4;
const F: usize = 5;
const G: usize = 6;
const H: usize = 7;
pub struct SHA256 {
    non_instance: PhantomData<bool>,
}

impl SHA256 {
    #[inline(always)]
    fn ch(x: u32, y: u32, z: u32) -> u32 {
        let a  = x & y;
        let not_x = !x;
        let b = not_x & z;
        return a ^ b;
    }

    #[inline(always)]
    fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    #[inline(always)]
    fn mix(w: &[u32], t: usize) -> u32 {
        let s0 = rotate_right(w[t - 15], 7)
            ^ rotate_right(w[t - 15], 18)
            ^ shift_right(w[t - 15], 3);


        let s1 = rotate_right(w[t - 2], 17)
            ^ rotate_right(w[t - 2], 19)
            ^ shift_right(w[t - 2], 10);

        mod_sum(&[s0, w[t - 7], s1, w[t - 16]])
    }

    #[inline(always)]
    fn chunk_to_schedule(w: &mut [u32], chunk: &[u8]){
        for i in (0..64).step_by(4){
            w[i / 4] = u32::from_be_bytes(
                [chunk[i+3], chunk[i+2], chunk[i+1], chunk[i]]
            ).to_be();
        }
    }

    /// Ensure that the message is a multiple of 512 bits.
    ///
    /// Append the bit 1 to the end of the message, followed by k zero bits,
    /// where k is the smallest, non-negative solution to the equation
    /// `l + 1 + k = 448 mod 512` => `l + 1000 0000 b + k = 56 mod 64`
    /// Then append the 64-bit block that is equal to the number L expressed using a binary representation
    fn do_chunk(hash: &mut [u32], chunk: &[u8], already_processed: u64, total_lenght: u64) {
        if chunk.len() < 56 {
            // This means that it needs padding but it can be padded in the same block
            // it needs to first 56 bits in order to avoid modifying the original data
            let mut padded = [0u8; 64];
            //Copy the data
            padded[..chunk.len()].copy_from_slice(chunk);
            padded[chunk.len()] = b'\x80';
            padded[56..64].copy_from_slice(&(total_lenght << 3).to_be_bytes());
            Self::do_64bytes_chunk(hash, &padded);
        } else if chunk.len() < 64 {
            // This means that it needs padding, but it should be added as a
            // new block because the len of the message can't be inserted in it
            let mut padded = [0u8; 64];
            padded[..chunk.len()].copy_from_slice(chunk);
            padded[chunk.len()] = b'\x80';

            Self::do_64bytes_chunk(hash, &padded);

            let mut padding = [0u8; 64];
            padding[56..64].copy_from_slice(&(total_lenght << 3).to_be_bytes());

            Self::do_64bytes_chunk(hash, &padding);
        } else if chunk.len() == 64 && already_processed + 64 >= total_lenght {
            // This means is the last chunk, but the bits of the message is divisible by 512.
            // Add a whole block of padding
            Self::do_64bytes_chunk(hash, chunk);
            let mut padding = [0u8; 64];
            padding[0] = b'\x80';
            padding[56..64].copy_from_slice(&(total_lenght << 3).to_be_bytes());

            Self::do_64bytes_chunk(hash, &padding);
        } else {
            Self::do_64bytes_chunk(hash, chunk);
        }
    }


    fn do_64bytes_chunk(hash: &mut [u32], chunk: &[u8]) {
        let mut w = [0u32;64];

        Self::chunk_to_schedule(&mut w, chunk);

        for t in 16..64 {
            w[t] = Self::mix(&w, t);
        }

        Self::hash_round(&w, hash);
    }

    fn hash_round(w: &[u32], hash_state: &mut [u32]) {
        let mut a = hash_state[A];
        let mut b = hash_state[B];
        let mut c = hash_state[C];
        let mut d = hash_state[D];
        let mut e = hash_state[E];
        let mut f = hash_state[F];
        let mut g = hash_state[G];
        let mut h = hash_state[H];

        for i in 0..64 {
            let ch = Self::ch(e, f, g);
            let maj = Self::maj(a, b, c);
            let s0 =
                rotate_right(a, 2) ^ rotate_right(a, 13) ^ rotate_right(a, 22);
            let s1 =
                rotate_right(e, 6) ^ rotate_right(e, 11) ^ rotate_right(e, 25);

            let t1 = mod_sum(&[h, s1, ch, w[i], CONSTANTS[i]]);
            h = g;
            g = f;
            f = e;
            e = mod_sum(&[d, t1]);
            d = c;
            c = b;
            b = a;
            a = mod_sum(&[t1, s0, maj]);
        }

        hash_state[A] = mod_sum(&[a, hash_state[A]]);
        hash_state[B] = mod_sum(&[b, hash_state[B]]);
        hash_state[C] = mod_sum(&[c, hash_state[C]]);
        hash_state[D] = mod_sum(&[d, hash_state[D]]);
        hash_state[E] = mod_sum(&[e, hash_state[E]]);
        hash_state[F] = mod_sum(&[f, hash_state[F]]);
        hash_state[G] = mod_sum(&[g, hash_state[G]]);
        hash_state[H] = mod_sum(&[h, hash_state[H]]);
    }
}

impl CryptoHasher for SHA256 {
    fn hash(bytes: &[u8]) -> CryptoHash {
        let mut hash =  [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

        if bytes.len() == 0 {
            let mut empty_chunk = [0u8;64];
            empty_chunk[0] = b'\x80';
            Self::do_64bytes_chunk(&mut hash, &empty_chunk);
        }

        for i in (0..bytes.len()).step_by(64) {
            let offset = min(i + 64, bytes.len());
            Self::do_chunk(
                &mut hash, 
                &bytes[i..offset], 
                i as u64, 
                bytes.len() as u64
            );
        }

        return CryptoHash::new_32bit_word(&hash, 256).expect("This should never happen");
    }
}

#[cfg(test)]
mod test{
    use crate::{encoding::{Digestable, hex::Hex}, hashers::Hashable};

    use super::*;

    #[test]
    fn ch_test(){
        assert_eq!(SHA256::ch(1u32, 2u32, 3u32), (1u32 & 2u32) ^ (!1u32 & 3u32));
        assert_eq!(SHA256::ch(1234125u32, 2211234u32, 1234123u32), (1234125u32 & 2211234u32) ^ (!1234125u32 & 1234123u32));
    }

    #[test]
    fn maj_test(){

    }

    #[test]
    fn sha256_test(){
        let inputs = [
            (
                b"This is a test".as_slice(),
                "c7be1ed902fb8dd4d48997c6452f5d7e509fbcdbe2808b16bcf4edce4c07d14e"
            ),
            (
                b"This is another test with a waaaaay longer input string, should be enough if this two are ok, because of the % chance of this two being the same but the rest is slim".as_slice(),
                "3ae593f415aa6e2f8016a1f85d72f9ef745d33490b1b8704b77f4b78f27634e7"
            )
        ];
        assert_eq!(SHA256::hash(inputs[0].0).digest::<Hex>().to_lowercase(), inputs[0].1);
        assert_eq!(SHA256::hash(inputs[1].0).digest::<Hex>().to_lowercase(), inputs[1].1);
    }

    #[test]
    fn sha256_test_impls(){
        let inputs = [
            (
                "This is a test",
                "c7be1ed902fb8dd4d48997c6452f5d7e509fbcdbe2808b16bcf4edce4c07d14e"
            ),
            (
                "This is another test with a waaaaay longer input string, should be enough if this two are ok, because of the % chance of this two being the same but the rest is slim",
                "3ae593f415aa6e2f8016a1f85d72f9ef745d33490b1b8704b77f4b78f27634e7"
            ),
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            ),
            (
                "abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
            ),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
            ),

        ];
        for input in inputs{
            assert_eq!(input.0.hash::<SHA256>().digest::<Hex>().to_lowercase(), input.1);
        }
    }
}
