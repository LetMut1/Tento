use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use std::default::Default;

pub struct HS512Encoder {
    key: &'static str
}

impl<'a, 'b: 'a> HS512Encoder {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn encode(&'a self, subject: &'b String) -> String {
        let mut hmac: Hmac<Sha512> = self.get_configured_hmac();
        hmac.input(subject.as_bytes());

        return hex::encode(hmac.result().code());
    }

    pub fn hash_is_valid(&'a self, subject: &'b String, subject_hash: &'b String) -> bool {
        return &self.encode(subject) == subject_hash;
    }

    fn get_configured_hmac(&'a self) -> Hmac<Sha512> {
        return Hmac::new(Sha512::new(), self.key.as_bytes());
    }
}

impl Default for HS512Encoder {
    fn default() -> Self {
        return Self {
            key: "Jd2DffsFi3sc8HMz2udvb71Hyz8zH12asfgu4S9DdsijS1ksS2v8cs5fFm6dcfLk"
        };
    }
}