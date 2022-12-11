use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ExpirationTimeResolver;

impl ExpirationTimeResolver {
    pub fn is_expired<'a>(
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(application_user_access_refresh_token.get_expires_at());
    }
}