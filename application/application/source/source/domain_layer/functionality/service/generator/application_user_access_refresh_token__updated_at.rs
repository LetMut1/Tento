use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt,
    infrastructure_layer::functionality::service::resolver::{
        date_time::UnixTime,
        Resolver,
    },
};
impl Generator<ApplicationUserAccessRefreshToken_UpdatedAt> {
    pub fn generate() -> i64 {
        return Resolver::<UnixTime>::get_now();
    }
}
