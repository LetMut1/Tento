use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationToken_ExpirationTimeResolver;

impl ApplicationUserRegistrationToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_registration_token: &'a ApplicationUserRegistrationToken<'_>) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(application_user_registration_token.get_expires_at());
    }
}