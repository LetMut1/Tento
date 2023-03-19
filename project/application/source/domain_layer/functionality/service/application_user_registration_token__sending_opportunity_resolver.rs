use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserRegistrationToken_SendingOpportunityResolver;

impl ApplicationUserRegistrationToken_SendingOpportunityResolver {
    pub fn can_send<'a>(application_user_registration_token: &'a ApplicationUserRegistrationToken<'_>) -> bool {
        return DateTimeResolver::is_greater_or_equal_than_now(
            application_user_registration_token.get_can_be_resent_from()
        );
    }
}