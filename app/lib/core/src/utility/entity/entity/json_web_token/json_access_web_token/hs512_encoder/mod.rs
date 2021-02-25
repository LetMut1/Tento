use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct HS512Encoder;

impl<'b> HS512Encoder {
    const PRIVATE_KEY: &'static str = "Jd2DffsFi3sc8HMz2udvb71Hyz8zH12asfgu4S9DdsijS1ksS2v8cs5fFm6dcfLk";

    pub fn encode(subject: &'b String) -> String {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac();
        hmac.input(subject.as_bytes());

        return hex::encode(hmac.result().code());
    }

    pub fn hash_is_valid(subject: &'b String, subject_hash: &'b String) -> bool {
        return &Self::encode(subject) == subject_hash;  // TODO // TODO // TODO сравнить значени строк, а не участки памяти, гна которые указывает сфлка 
    }

    fn get_configured_hmac() -> Hmac<Sha512> {
        return Hmac::new(Sha512::new(), Self::PRIVATE_KEY.as_bytes());
    }
}