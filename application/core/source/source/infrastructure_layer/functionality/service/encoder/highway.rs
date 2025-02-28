use {
    super::Encoder,
    highway::{
        HighwayHasher,
        HighwayHash,
        Key,
    },
};
pub struct Highway;
impl Encoder<Highway> {
    pub fn encode<'a>(key: [u64; 4], data_for_encode: &'a [u8]) -> u64 {
        return Self::prepare(key).hash64(data_for_encode);
    }
    pub fn is_valid<'a>(key: [u64; 4], data_for_encode: &'a [u8], encoded_data: u64) -> bool {
        return Self::prepare(key).hash64(data_for_encode) == encoded_data;
    }
    fn prepare(key: [u64; 4]) -> HighwayHasher {
        return HighwayHasher::new(
            Key(key),
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn validate() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let key = [0 as u64, 1, 2, 3];
        let data_for_encode = [0 as u8, 1];
        if !Encoder::<Highway>::is_valid(
            key,
            data_for_encode.as_slice(),
            Encoder::<Highway>::encode(key, data_for_encode.as_slice()),
        ) {
            return Result::Err("".into());
        }
        return Result::Ok(());
    }
}