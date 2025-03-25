use {
    super::Generator,
    crate::{
        domain_layer::data::entity::channel_publication1_commentary::ChannelPublication1Commentary_CanBeDeletedFrom,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<ChannelPublication1Commentary_CanBeDeletedFrom> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval(
            ChannelPublication1Commentary_CanBeDeletedFrom::QUANTITY_OF_MICROSECONDS_BEFORE_DELETION,
            now,
        );
    }
}
