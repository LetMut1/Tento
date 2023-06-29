use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;

impl Generator<ApplicationUserRegistrationToken_CanBeResentFrom> {
    pub fn generate() -> Result<ApplicationUserRegistrationToken_CanBeResentFrom, ErrorAuditor> {
        let application_user_registration_token_can_be_resent_from = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING) {
            Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(ApplicationUserRegistrationToken_CanBeResentFrom::new(application_user_registration_token_can_be_resent_from));
    }
}
