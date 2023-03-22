use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAuthorizationToken_SendingOpportunityResolver;

impl ApplicationUserAuthorizationToken_SendingOpportunityResolver {
    pub fn can_send<'a>(application_user_authorization_token: &'a ApplicationUserAuthorizationToken<'_>) -> bool {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(
            application_user_authorization_token.get_can_be_resent_from()
        );
    }
}