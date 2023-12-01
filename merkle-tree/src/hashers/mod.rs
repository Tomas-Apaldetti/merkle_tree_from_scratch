pub mod sha256;
pub(super) mod utils;

pub struct CryptoHash {
    pub(crate) data: [u32; 8],
}

pub trait CryptoHasher {
    fn hash(bytes: &[u8]) -> CryptoHash;
}
