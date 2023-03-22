use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAuthorizationToken_ExpirationTimeResolver;

impl ApplicationUserAuthorizationToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_authorization_token: &'a ApplicationUserAuthorizationToken<'_>) -> bool {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(application_user_authorization_token.get_expires_at());
    }
}