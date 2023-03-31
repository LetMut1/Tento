use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::functionality::service::number_row_generator::NumberRowGenerator;
use extern_crate::uuid::Uuid;
use std::marker::PhantomData;

pub struct Generator<S> {
    _subject: PhantomData<S>
}

impl Generator<ApplicationUserAccessToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_access_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserAccessToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_access_token_expires_at_) => application_user_access_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_access_token_expires_at);
    }
}

impl Generator<ApplicationUserAccessToken_Id> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}

impl Generator<ApplicationUserAccessRefreshToken_ObfuscationValue> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}

impl Generator<ApplicationUserAccessRefreshToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_access_refresh_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserAccessRefreshToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_access_refresh_token_expires_at);
    }
}

impl Generator<ApplicationUserAccessRefreshToken_UpdatedAt> {
    pub fn generate() -> i64 {
        return DateTimeResolver::unixtime_get_now();
    }
}

impl Generator<ApplicationUserAuthorizationToken_Value> {
    pub fn generate() -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}

impl Generator<ApplicationUserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_authorization_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserAuthorizationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_authorization_token_expires_at);
    }
}

impl Generator<ApplicationUserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_authorization_token_can_be_resent_from = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
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

impl Generator<ApplicationUserRegistrationToken_Value> {
    pub fn generate() -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}

impl Generator<ApplicationUserRegistrationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_registration_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
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
        let application_user_registration_token_can_be_resent_from = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
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

impl Generator<ApplicationUserResetPasswordToken_Value> {
    pub fn generate() -> String {
        return NumberRowGenerator::generate_row_with_6_numbers();
    }
}

impl Generator<ApplicationUserResetPasswordToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_reset_password_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_reset_password_token_expires_at);
    }
}

impl Generator<ApplicationUserResetPasswordToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_reset_password_token_can_be_resent_from = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING
        ) {
            Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_reset_password_token_can_be_resent_from);
    }
}