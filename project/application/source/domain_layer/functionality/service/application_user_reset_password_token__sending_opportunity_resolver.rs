use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserResetPasswordToken_SendingOpportunityResolver;

impl ApplicationUserResetPasswordToken_SendingOpportunityResolver {
    pub fn can_send<'a>(application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>) -> bool {
        return DateTimeResolver::is_greater_or_equal_than_now(
            application_user_reset_password_token.get_can_be_resent_from()
        );
    }
}