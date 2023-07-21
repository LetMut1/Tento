use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;

impl Generator<ApplicationUserAccessRefreshToken_UpdatedAt> {
    pub fn generate() -> ApplicationUserAccessRefreshToken_UpdatedAt {
        return ApplicationUserAccessRefreshToken_UpdatedAt(Resolver::<DateTime>::unixtime_get_now());
    }
}
