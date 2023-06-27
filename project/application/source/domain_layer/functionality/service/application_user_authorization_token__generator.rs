use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::NumberRow;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;

impl Generator<ApplicationUserAuthorizationToken_Value> {
    pub fn generate() -> ApplicationUserAuthorizationToken_Value {
        return ApplicationUserAuthorizationToken_Value::new(Generator_::<NumberRow>::generate_6());
    }
}

impl Generator<ApplicationUserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserAuthorizationToken_ExpiresAt, ErrorAuditor> {
        let application_user_authorization_token_expires_at =
            match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
                ApplicationUserAuthorizationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION,
            ) {
                Ok(application_user_authorization_token_expires_at_) => {
                    application_user_authorization_token_expires_at_
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

        return Ok(ApplicationUserAuthorizationToken_ExpiresAt::new(application_user_authorization_token_expires_at));
    }
}

impl Generator<ApplicationUserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<ApplicationUserAuthorizationToken_CanBeResentFrom, ErrorAuditor> {
        let application_user_authorization_token_can_be_resent_from =
            match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
                ApplicationUserAuthorizationToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING,
            ) {
                Ok(application_user_authorization_token_can_be_resent_from_) => {
                    application_user_authorization_token_can_be_resent_from_
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
            ApplicationUserAuthorizationToken_CanBeResentFrom::new(
                application_user_authorization_token_can_be_resent_from,
            ),
        );
    }
}
