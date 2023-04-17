use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::argon2::Config as ArgonConfig;
use extern_crate::argon2::hash_encoded as argon2_hash_encoded;
use extern_crate::argon2::verify_encoded as argon2_verify_encoded;
use extern_crate::base64::Config as Base64Config;
use extern_crate::base64::decode_config as base64_decode_config;
use extern_crate::base64::encode_config as base64_encode_config;
use extern_crate::base64::STANDARD;
use extern_crate::crypto::hmac::Hmac as Hmac_;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use extern_crate::hex::encode as hex_encode;
use extern_crate::uuid::Uuid;
use std::marker::PhantomData;

pub struct Encoder<S> {
    _subject: PhantomData<S>
}

pub struct Argon2Id;

pub struct Hmac;

pub struct Hex;

pub struct Base64;

impl Encoder<Argon2Id> {       // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090
    pub fn encode<'a>(data: &'a [u8]) -> Result<String, ErrorAuditor> {    // TODO TODO TODO ARGON2id . ПРОВЕрИТЬЬ, он или нет, понять, почему не он.
        let config = ArgonConfig::default();   // TODO настроить конфиг, возможно, вынестки в константу

        let salt = Uuid::new_v4();

        let value = match argon2_hash_encoded(data, salt.as_bytes().as_slice(), &config) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(value);
    }

    pub fn is_valid<'a>(data: &'a [u8], encoded_data: &'a str) -> Result<bool, ErrorAuditor> {
        let value = match argon2_verify_encoded(encoded_data, data) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(value);
    }
}

impl Encoder<Hmac> {
    pub fn encode<'a>(salt: &'a [u8], data: &'a [u8], encoded_data: &'a mut [u8]) -> () {
        let mut hmac = Hmac_::new(Sha512::new(), salt);

        hmac.input(data);

        hmac.raw_result(encoded_data);  // TODO TIme attack

        return ();
    }
}

impl Encoder<Hex> {
    pub fn encode<'a>(data: &'a [u8]) -> String {
        return hex_encode(data);
    }
}

impl Encoder<Base64> {
    const BASE64_STANDARD_CONFIGURATION: Base64Config = STANDARD;   // TODO подходит ли?  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?

    pub fn encode<'a>(data: &'a [u8]) -> String {
        return base64_encode_config(data, Self::BASE64_STANDARD_CONFIGURATION);
    }

    pub fn decode<'a>(encoded_data: &'a [u8]) -> Result<Vec<u8>, ErrorAuditor> {
        let data = match base64_decode_config(encoded_data, Self::BASE64_STANDARD_CONFIGURATION) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(data);
    }
}