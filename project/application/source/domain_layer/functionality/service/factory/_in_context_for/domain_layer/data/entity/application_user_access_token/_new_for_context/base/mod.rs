use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use std::borrow::Cow;

pub struct Base;

impl Base {
    pub fn create_from_application_user_access_refresh_token<'a>(
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<ApplicationUserAccessToken<'a>, ErrorAuditor> {
        match DateTimeResolver::add_interval_from_now_formated(ApplicationUserAccessToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
            Ok(expiration_time) => {
                return Ok(
                    ApplicationUserAccessToken::new(
                        Cow::Borrowed(application_user_access_refresh_token.get_application_user_access_token_id()),
                        application_user_access_refresh_token.get_application_user_id(),
                        Cow::Borrowed(application_user_access_refresh_token.get_application_user_log_in_token_device_id()),
                        expiration_time
                    )
                );
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}