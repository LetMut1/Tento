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
use crate::domain_layer::data::entity::channel::Id as ChannelId;
use crate::domain_layer::data::entity::channel::LinkedName;
use crate::domain_layer::data::entity::channel::Name;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::regex::Regex;
use std::marker::PhantomData;

pub struct Validator<E, F> {
    _entity: PhantomData<E>,
    _field: PhantomData<F>
}

impl Validator<ApplicationUser<'_>, ApplicationUserId> {
    pub fn is_valid<'a>(application_user_id: i64) -> bool {
        return application_user_id >= 0;
    }
}

impl Validator<ApplicationUser<'_>, Email> {
    const REGULAR_EXPRESSION: &'static str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
    const MAXIMUM_LENGTH: usize = 320;

    pub fn is_valid<'a>(application_user_email: &'a str) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(Self::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            regex.is_match(application_user_email)
                && application_user_email.chars().count() <= Self::MAXIMUM_LENGTH
        );
    }
}

impl Validator<ApplicationUser<'_>, Nickname> {
    pub const MAXIMUM_LENGTH: usize = 55;

    pub fn is_valid<'a>(application_user_nickname: &'a str) -> bool {
        return application_user_nickname.chars().count() <= Self::MAXIMUM_LENGTH
            && !application_user_nickname.contains('@')
            && !application_user_nickname.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user_nickname.is_empty();
    }
}

impl Validator<ApplicationUser<'_>, Password> {
    const MINIMUM_LENGTH: usize = 7;
    const MAXIMUM_LENGTH: usize = 65;

    pub fn is_valid<'a>(application_user_password: &'a str) -> bool {
        let password_chars_count = application_user_password.chars().count();

        return password_chars_count >= Self::MINIMUM_LENGTH             // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
            && password_chars_count <= Self::MAXIMUM_LENGTH
            && !application_user_password.contains(' ');
    }
}

impl Validator<ApplicationUserAuthorizationToken<'_>, ApplicationUserAuthorizationTokenValue> {
    const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;

    pub fn is_valid<'a>(application_user_authorization_token_value: &'a str) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(Self::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            regex.is_match(application_user_authorization_token_value)
        );
    }
}

impl Validator<ApplicationUserRegistrationToken<'_>, ApplicationUserRegistrationTokenValue> {
    const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;

    pub fn is_valid<'a>(application_user_authorization_token_value: &'a str) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(Self::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            regex.is_match(application_user_authorization_token_value)
        );
    }
}

impl Validator<ApplicationUserResetPasswordToken<'_>, ApplicationUserResetPasswordTokenValue> {
    const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;

    pub fn is_valid<'a>(application_user_authorization_token_value: &'a str) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(Self::REGULAR_EXPRESSION) {
            Ok(regex_) => regex_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            regex.is_match(application_user_authorization_token_value)
        );
    }
}

impl Validator<ApplicationUserDevice, ApplicationUserDeviceId> {
    pub fn is_valid<'a>(application_user_device_id: &'a str) -> bool {
        return true;
    }
}

impl Validator<Channel<'_>, ChannelId> {
    pub fn is_valid<'a>(channel_id: i64) -> bool {
        return channel_id >= 0;
    }
}

impl Validator<Channel<'_>, Name> {
    pub const MAXIMUM_LENGTH: usize = 75;

    pub fn is_valid<'a>(channel_name: &'a str) -> bool {
        return channel_name.chars().count() <= Self::MAXIMUM_LENGTH
            && !channel_name.is_empty();
    }
}

impl Validator<Channel<'_>, LinkedName> {
    pub fn is_valid<'a>(channel_linked_name: &'a str) -> bool {
        return true;    // TODO;
    }
}