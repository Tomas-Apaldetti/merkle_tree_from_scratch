pub mod sha256;
pub(super) mod utils;

pub(super) struct CryptoHash {
    data: [u32; 8],
}

pub trait CryptoHasher {
    fn hash(bytes: &[u8]) -> CryptoHash;
}
