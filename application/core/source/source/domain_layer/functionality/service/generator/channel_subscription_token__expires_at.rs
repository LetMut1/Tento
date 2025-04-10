use {
    super::Generator,
    crate::{
        domain_layer::data::entity::channel_subscription_token::ChannelSubscriptionToken_ExpiresAt,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<ChannelSubscriptionToken_ExpiresAt> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval_microseconds(
            ChannelSubscriptionToken_ExpiresAt::QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION,
            now,
        );
    }
}
