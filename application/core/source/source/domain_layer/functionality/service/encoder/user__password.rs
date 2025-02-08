use {
    crate::{
        domain_layer::data::entity::user::User_Password,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::encoder::{
                Argon2Id,
                Encoder as Encoder_,
            },
        },
    },
    super::Encoder,
};
impl Encoder<User_Password> {
    pub fn encode<'a>(user__password: &'a str) -> Result<String, AggregateError> {
        return Encoder_::<Argon2Id>::encode(user__password.as_bytes());
    }
    pub fn is_valid<'a>(user__password: &'a str, user__password_hash: &'a str) -> Result<bool, AggregateError> {
        return Encoder_::<Argon2Id>::is_valid(
            user__password.as_bytes(),
            user__password_hash,
        );
    }
}
