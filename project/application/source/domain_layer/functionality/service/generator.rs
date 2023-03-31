use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ObfuscationValue as ApplicationUserAccessRefreshTokenObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ExpiresAt as ApplicationUserAccessTokenExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::Id as ApplicationUserAccessTokenId;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::Value as ApplicationUserAuthorizationTokenValue;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::Id as ApplicationUserDeviceId;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::Value as ApplicationUserRegistrationTokenValue;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::Value as ApplicationUserResetPasswordTokenValue;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::Email;
use crate::domain_layer::data::entity::application_user::Id as ApplicationUserId;
use crate::domain_layer::data::entity::application_user::Nickname;
use crate::domain_layer::data::entity::application_user::Password;
use crate::domain_layer::data::entity::channel::Channel;
use crate::domain_layer::data::entity::channel::Description;
use crate::domain_layer::data::entity::channel::Id as ChannelId;
use crate::domain_layer::data::entity::channel::LinkedName;
use crate::domain_layer::data::entity::channel::Name;
use crate::domain_layer::data::entity::channel::Orientation;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use extern_crate::regex::Regex;
use extern_crate::uuid::Uuid;
use std::marker::PhantomData;

pub struct Generator<E, F> {
    _entity: PhantomData<E>,
    _field: PhantomData<F>
}

impl Generator<ApplicationUserAccessToken<'_>, ApplicationUserAccessTokenExpiresAt> {
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

impl Generator<ApplicationUserAccessToken<'_>, ApplicationUserAccessTokenId> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}

impl Generator<ApplicationUserAccessRefreshToken<'_>, ApplicationUserAccessRefreshTokenObfuscationValue> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}
