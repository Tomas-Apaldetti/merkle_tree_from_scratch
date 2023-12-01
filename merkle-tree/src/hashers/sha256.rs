use std::{cmp::min, marker::PhantomData};

use super::{utils::mod_sum, CryptoHash, CryptoHasher};

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

const WORD_LENGTH_BITS: u32 = 32;
pub struct SHA256 {
    non_instance: PhantomData<bool>,
}

impl SHA256 {
    /// Circula right shift operation, where `x` is a 32 bit word
    /// and n is an integer with 0 <= n < w (not checked at runtime)
    fn rotate_right(x: u32, n: u8) -> u32 {
        (x >> n) & (x << (WORD_LENGTH_BITS - n as u32))
    }

    /// Right shift operation where `x` is a 32 bit word,
    /// and n is an integer with 0 <= n < w (not checked at runtime)
    fn shif_right(x: u32, n: u8) -> u32 {
        x >> n
    }

    fn ch(x: u32, y: u32, z: u32) -> u32 {
        let a: u32 = x & y;
        let not_x = !x;
        let b = not_x & z;
        return a ^ b;
    }

    fn maj(x: u32, y: u32, z: u32) -> u32 {
        return (x & y) ^ (x & z) ^ (y ^ z);
    }

    fn mix(w: &[u32], t: usize) -> u32 {
        let s0 = Self::rotate_right(w[t - 2], 7)
            ^ Self::rotate_right(w[t - 2], 18)
            ^ Self::shif_right(w[t - 2], 3);
        let s1 = Self::rotate_right(w[t - 15], 7)
            ^ Self::rotate_right(w[t - 15], 18)
            ^ Self::shif_right(w[t - 15], 3);

        mod_sum(&[s0, w[t - 7], s1, w[t - 16]])
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
                Self::rotate_right(a, 2) ^ Self::rotate_right(a, 13) ^ Self::rotate_right(a, 22);
            let s1 =
                Self::rotate_right(e, 6) ^ Self::rotate_right(e, 11) ^ Self::rotate_right(e, 25);

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

    fn do_64bytes_chunk(hash: &mut [u32], chunk: &[u8]) {
        let mut message_schedule = [0u8; 64 * 4];
        message_schedule[..(16 * 4)].copy_from_slice(&chunk);
        // Safety: Fixed length arrays, they are the same array interpreted another way
        let mut w: [u32; 64] = unsafe { std::mem::transmute(message_schedule) };
        for t in 16..64 {
            w[t] = Self::mix(&w, t);
        }

        Self::hash_round(&w, hash);
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
            padded[56..64].copy_from_slice(&total_lenght.to_be_bytes());
            Self::do_64bytes_chunk(hash, &padded);
        } else if chunk.len() < 64 {
            // This means that it needs padding, but it should be added as a
            // new block because the len of the message can't be inserted in it
            let mut padded = [0u8; 64];
            padded[..chunk.len()].copy_from_slice(chunk);
            padded[chunk.len()] = b'\x80';

            Self::do_64bytes_chunk(hash, &padded);

            let mut padding = [0u8; 64];
            padding[56..64].copy_from_slice(&total_lenght.to_be_bytes());

            Self::do_64bytes_chunk(hash, &padding);
        } else if chunk.len() == 64 && already_processed + 64 >= total_lenght {
            // This means is the last chunk, but the bits of the message is divisible by 512.
            // Add a whole block of padding
            Self::do_64bytes_chunk(hash, chunk);
            let mut padding = [0u8; 64];
            padding[0] = b'\x80';
            padding[56..64].copy_from_slice(&total_lenght.to_be_bytes());

            Self::do_64bytes_chunk(hash, &padding);
        } else {
            Self::do_64bytes_chunk(hash, chunk);
        }
    }
}

impl CryptoHasher for SHA256 {
    fn hash(bytes: &[u8]) -> CryptoHash {
        let mut hash =  [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

        for i in (0..bytes.len()).step_by(64) {
            let offset = min(i + 64, bytes.len());
            Self::do_chunk(&mut hash, &bytes[i..offset], i as u64, bytes.len() as u64)
        }

        return CryptoHash { data: hash };
    }
}
