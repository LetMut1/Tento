use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

#[allow(non_camel_case_types)]
pub struct ApplicationUserLogInToken_ExpirationTimeResolver;

impl ApplicationUserLogInToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_log_in_token: &'a ApplicationUserLogInToken<'_>) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(application_user_log_in_token.get_expires_at());
    }
}