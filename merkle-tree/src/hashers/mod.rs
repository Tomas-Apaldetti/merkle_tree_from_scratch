pub mod impls;

pub mod sha256;
pub(super) mod utils;

pub struct CryptoHash {
    pub(crate) data: Vec<u8>,
}

pub trait CryptoHasher {
    fn hash(bytes: &[u8]) -> CryptoHash;
}

pub trait Hashable{
    fn to_bits(&self) -> &[u8];

    fn hash<T: CryptoHasher>(&self) -> CryptoHash{
        T::hash(self.to_bits())
    }
}

impl Hashable for str{
    fn to_bits(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Hashable for String{
    fn to_bits(&self) -> &[u8] {
        self.as_bytes()
    }
}