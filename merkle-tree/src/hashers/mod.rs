pub mod sha256;

pub(super) struct CryptoHash{
    data: [u32; 8]
}

pub trait CryptoHasher{
    fn hash(&self, bytes: &[u8]) -> CryptoHash;
}