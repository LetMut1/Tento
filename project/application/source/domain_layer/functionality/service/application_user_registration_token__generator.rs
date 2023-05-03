use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::NumberRow;
use super::generator::Generator;

impl Generator<ApplicationUserRegistrationToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}

impl Generator<ApplicationUserRegistrationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_registration_token_expires_at = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
            ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_registration_token_expires_at);
    }
}

impl Generator<ApplicationUserRegistrationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_registration_token_can_be_resent_from = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
            ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING
        ) {
            Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_registration_token_can_be_resent_from);
    }
}