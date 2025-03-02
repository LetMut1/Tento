use {
    super::Generator,
    crate::{
        domain_layer::data::entity::channel_token::ChannelToken_ExpiresAt,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<ChannelToken_ExpiresAt> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval(
            ChannelToken_ExpiresAt::QUANTITY_OF_SECONDS_FOR_EXPIRATION,
            now,
        );
    }
}
