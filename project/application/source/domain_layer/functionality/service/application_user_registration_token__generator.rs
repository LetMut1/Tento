use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::NumberRow;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;

impl Generator<ApplicationUserRegistrationToken_Value> {
    pub fn generate() -> ApplicationUserRegistrationToken_Value {
        return ApplicationUserRegistrationToken_Value::new(Generator_::<NumberRow>::generate_6());
    }
}

impl Generator<ApplicationUserRegistrationToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserRegistrationToken_ExpiresAt, ErrorAuditor> {
        let application_user_registration_token_expires_at =
            match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
                ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION,
            ) {
                Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
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

        return Ok(ApplicationUserRegistrationToken_ExpiresAt::new(application_user_registration_token_expires_at));
    }
}

impl Generator<ApplicationUserRegistrationToken_CanBeResentFrom> {
    pub fn generate() -> Result<ApplicationUserRegistrationToken_CanBeResentFrom, ErrorAuditor> {
        let application_user_registration_token_can_be_resent_from =
            match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
                ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING,
            ) {
                Ok(application_user_registration_token_can_be_resent_from_) => {
                    application_user_registration_token_can_be_resent_from_
                }
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

        return Ok(
            ApplicationUserRegistrationToken_CanBeResentFrom::new(
                application_user_registration_token_can_be_resent_from,
            ),
        );
    }
}
