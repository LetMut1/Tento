use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAuthorizationToken_SendingOpportunityResolver;

impl ApplicationUserAuthorizationToken_SendingOpportunityResolver {
    pub fn can_send<'a>(application_user_authorization_token: &'a ApplicationUserAuthorizationToken<'_>) -> bool {
        return !DateTimeResolver::is_greater_or_equal_than_now(
            application_user_authorization_token.get_can_be_resent_from()
        );
    }

    pub fn create_can_be_resent_from() -> Result<i64, ErrorAuditor> {
        let application_user_authorization_token_can_be_resent_from = match DateTimeResolver::add_minutes_interval_from_now(
            ApplicationUserAuthorizationToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING
        ) {
            Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_authorization_token_can_be_resent_from);
    }
}