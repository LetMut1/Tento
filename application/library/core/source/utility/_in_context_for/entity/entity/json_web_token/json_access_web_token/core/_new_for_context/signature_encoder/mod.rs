use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureEncoder;

impl<'outer> SignatureEncoder {
    const PRIVATE_KEY: &'static str = "Jd2DffsFi3sc8Mz2udB0qRbz8zH12asu4S1ksSsZ6e2v8cs5fFm6dcfLk";  // TODO где должен быть ключ

    pub fn encode(subject: &'outer str) -> String {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac();
        
        hmac.input(subject.as_bytes());

        return hex::encode(hmac.result().code());
    }

    pub fn is_valid(subject: &'outer str, subject_signature: &'outer str) -> bool {
        return Self::encode(subject) == *subject_signature;
    }

    fn get_configured_hmac() -> Hmac<Sha512> {
        return Hmac::new(Sha512::new(), Self::PRIVATE_KEY.as_bytes());
    }
}