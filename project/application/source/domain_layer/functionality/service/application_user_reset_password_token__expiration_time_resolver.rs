use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserResetPasswordToken_ExpirationTimeResolver;

impl ApplicationUserResetPasswordToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_reset_password_token: &'a ApplicationUserResetPasswordToken) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(application_user_reset_password_token.get_expires_at());
    }
}