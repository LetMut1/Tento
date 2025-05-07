use {
    super::Generator,
    crate::{
        domain_layer::data::entity::user::User_ObfuscatedId,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<User_ObfuscatedId> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::get_now_in_nanoseconds();
    }
}
