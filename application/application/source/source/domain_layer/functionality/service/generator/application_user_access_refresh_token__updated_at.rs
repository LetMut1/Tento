use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt,
    infrastructure_layer::{
        data::control_type::DateTime,
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserAccessRefreshToken_UpdatedAt> {
    pub fn generate() -> i64 {
        return Resolver::<DateTime>::unixtime_get_now();
    }
}
