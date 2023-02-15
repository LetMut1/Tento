use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAccessToken_ExpirationTimeResolver;

impl ApplicationUserAccessToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_access_token: &'a ApplicationUserAccessToken<'_>) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(application_user_access_token.get_expires_at());
    }
}