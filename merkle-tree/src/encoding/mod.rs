pub mod hex;
pub mod base64;

pub trait Digester{
    fn digest(bits: &[u8]) -> String;
}

pub trait Digestable{
    fn bits(&self) -> &[u8];

    fn digest<T: Digester>(&self) -> String{
        T::digest(self.bits())
    }
}
