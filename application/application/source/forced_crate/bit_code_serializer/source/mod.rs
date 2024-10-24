use bitcode::{
    Encode,
    Decode,
    Error,
};
pub struct Serializer;
impl Serializer {
    pub fn serialize<'a, T>(subject: &'a T) -> Vec<u8>
    where
        T: Encode,
    {
        return bitcode::encode(subject);
    }
    pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Error>
    where
        T: Decode<'a>,
    {
        return bitcode::decode::<'_, T>(data);
    }
}
