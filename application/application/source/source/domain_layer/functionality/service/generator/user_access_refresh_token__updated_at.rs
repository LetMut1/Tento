use super::Generator;
use crate::{
    domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken_UpdatedAt,
    infrastructure_layer::functionality::service::resolver::{
        UnixTime,
        Resolver,
    },
};
impl Generator<UserAccessRefreshToken_UpdatedAt> {
    pub fn generate() -> i64 {
        return Resolver::<UnixTime>::get_now();
    }
}
