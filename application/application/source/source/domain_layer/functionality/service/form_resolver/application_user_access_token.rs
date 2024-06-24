use super::FormResolver;
use crate::{
    domain_layer::{
        data::entity::application_user_access_token::ApplicationUserAccessToken,
        functionality::service::encoder::Encoder,
    },
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
            },
            control_type::{
                Base64,
                MessagePack,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
        },
        functionality::service::{
            encoder::{
                hmac::HmacSha3_512,
                Encoder as Encoder_,
            },
            serializer::{
                Serialize,
                Serializer,
            },
        },
    },
};
impl FormResolver<ApplicationUserAccessToken<'_>> {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";
    pub fn to_encrypted<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_token: &'a ApplicationUserAccessToken<'_>,
    ) -> Result<String, Auditor<Error>> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_token)?;
        let application_user_access_token_serialized = Encoder_::<Base64>::encode(data.as_slice());
        let application_user_access_token_serialized_signature = Encoder::<Signature>::encode(
            environment_configuration,
            application_user_access_token_serialized.as_bytes(),
        )?;
        let application_user_access_token_encrypted = format!(
            "{}{}{}",
            application_user_access_token_serialized.as_str(),
            Self::TOKEN_PARTS_SEPARATOR,
            application_user_access_token_serialized_signature.as_str()
        );
        return Ok(application_user_access_token_encrypted);
    }
    pub fn from_encrypted<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_token_encrypted: &'a str,
    ) -> Result<Result<ApplicationUserAccessToken<'static>, Auditor<InvalidArgument>>, Auditor<Error>> {
        let mut token_part_registry = application_user_access_token_encrypted.splitn::<'_, &'_ str>(
            2,
            Self::TOKEN_PARTS_SEPARATOR,
        );
        let application_user_access_token_serialized = match token_part_registry.next() {
            Some(application_user_access_token_serialized_) => application_user_access_token_serialized_,
            None => {
                return Ok(
                    Err(
                        Auditor::<InvalidArgument>::new(
                            InvalidArgument,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    ),
                );
            }
        };
        let application_user_access_token_serialized_signature = match token_part_registry.next() {
            Some(application_user_access_token_serialized_signature_) => application_user_access_token_serialized_signature_,
            None => {
                return Ok(
                    Err(
                        Auditor::<InvalidArgument>::new(
                            InvalidArgument,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    ),
                );
            }
        };
        if !Encoder::<Signature>::is_valid(
            environment_configuration,
            application_user_access_token_serialized.as_bytes(),
            application_user_access_token_serialized_signature.as_bytes(),
        )? {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }
        let data = match Encoder_::<Base64>::decode(application_user_access_token_serialized.as_bytes()) {
            Ok(data_) => data_,
            Err(_) => {
                return Ok(
                    Err(
                        Auditor::<InvalidArgument>::new(
                            InvalidArgument,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    ),
                );
            }
        };
        return Ok(
            Ok(
                Serializer::<MessagePack>::deserialize::<'_, ApplicationUserAccessToken<'static>>(data.as_slice())?
            )
        );
    }
}
struct Signature;
impl Encoder<Signature> {
    fn encode<'a>(environment_configuration: &'a EnvironmentConfiguration, application_user_access_token_serialized: &'a [u8]) -> Result<String, Auditor<Error>> {
        let application_user_access_token_serialized_encoded = Encoder_::<HmacSha3_512>::encode(
            environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
            application_user_access_token_serialized,
        )?;
        let application_user_access_token_serialized_signature = Encoder_::<Base64>::encode(application_user_access_token_serialized_encoded.into_bytes().as_slice());
        return Ok(application_user_access_token_serialized_signature);
    }
    fn is_valid<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_token_serialized: &'a [u8],
        application_user_access_token_serialized_signature: &'a [u8],
    ) -> Result<bool, Auditor<Error>> {
        let application_user_access_token_serialized_encoded = Encoder_::<Base64>::decode(application_user_access_token_serialized_signature)?;
        return Ok(
            Encoder_::<HmacSha3_512>::is_valid(
                environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
                application_user_access_token_serialized,
                application_user_access_token_serialized_encoded.as_slice(),
            )?,
        );
    }
}
