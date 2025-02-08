use {
    super::Generator,
    crate::{
        domain_layer::data::entity::user_authorization_token::UserAuthorizationToken_ExpiresAt,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<UserAuthorizationToken_ExpiresAt> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval(
            UserAuthorizationToken_ExpiresAt::QUANTITY_OF_SECONDS_FOR_EXPIRATION,
            now,
        );
    }
}
