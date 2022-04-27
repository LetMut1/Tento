use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;
use std::borrow::Cow;

pub struct Base;

impl JsonAccessWebTokenFactoryTrait for Base {
    type Error = ErrorAuditor;

    fn create_from_json_refresh_web_token<'a>(
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<JsonAccessWebToken<'a>, Self::Error> {
        match DateTimeResolver::add_interval_from_now(&(JsonAccessWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64)) {
            Ok(expiration_time) => {
                return Ok(
                    Self::create(
                        Cow::Borrowed(json_refresh_web_token.get_json_access_web_token_id()),
                        Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
                        Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
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