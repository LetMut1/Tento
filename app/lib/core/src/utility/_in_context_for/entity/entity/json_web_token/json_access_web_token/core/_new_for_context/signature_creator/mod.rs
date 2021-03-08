use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl<'outer> SignatureCreator {
    const PRIVATE_KEY: &'static str = "Jd2DffsFi3sc8Mz2udB0bz8zH12asu4S1ksS2v8cs5fFm6dcfLk";

    pub fn encode(subject: &'outer String) -> String {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac();
        
        hmac.input(subject.as_bytes());

        return hex::encode(hmac.result().code());
    }

    pub fn hash_is_valid(subject: &'outer String, subject_hash: &'outer String) -> bool {
        return &Self::encode(subject) == subject_hash;
    }

    fn get_configured_hmac() -> Hmac<Sha512> {
        return Hmac::new(Sha512::new(), Self::PRIVATE_KEY.as_bytes());
    }
}