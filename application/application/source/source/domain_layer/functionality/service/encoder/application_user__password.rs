use super::Encoder;
use crate::{
    domain_layer::data::entity::user::User_Password,
    infrastructure_layer::functionality::service::encoder::{
        argon2id::Argon2Id,
        Encoder as Encoder_,
    },
};
use aggregate_error::AggregateError;
impl Encoder<User_Password> {
    pub fn encode<'a>(application_user__password: &'a str) -> Result<String, AggregateError> {
        return Encoder_::<Argon2Id>::encode(application_user__password.as_bytes());
    }
    pub fn is_valid<'a>(application_user__password: &'a str, application_user__password_hash: &'a str) -> Result<bool, AggregateError> {
        return Encoder_::<Argon2Id>::is_valid(
            application_user__password.as_bytes(),
            application_user__password_hash,
        );
    }
}
