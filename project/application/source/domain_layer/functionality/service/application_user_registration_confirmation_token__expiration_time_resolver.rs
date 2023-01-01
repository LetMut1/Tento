use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

#[allow(non_camel_case_types)]
pub struct ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver;

impl ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(application_user_registration_confirmation_token.get_expires_at());
    }
}