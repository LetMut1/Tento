use {
    super::Generator,
    crate::{
        domain_layer::data::entity::channel_publication1_commentary_token::ChannelPublication1CommentaryToken_ExpiresAt,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<ChannelPublication1CommentaryToken_ExpiresAt> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval_microseconds(
            ChannelPublication1CommentaryToken_ExpiresAt::QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION,
            now,
        );
    }
}
