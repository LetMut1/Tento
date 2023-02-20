use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::regex::Regex;

pub struct ApplicationUser_Validator;

impl ApplicationUser_Validator {
    const APPLICATION_USER__EMAIL_MAXIMUM_LENGTH: usize = 320;
    const APPLICATION_USER__NICKNAME_MAXIMUM_LENGTH: usize = 55;
    const APPLICATION_USER__PASSWORD_MINIMUM_LENGTH: usize = 7;
    const APPLICATION_USER__PASSWORD_MAXIMUM_LENGTH: usize = 65;
    const APPLICATION_USER__EMAIL_REGULAR_EXPRESSION: &'static str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;

    pub fn is_valid_email<'a>(application_user_email: &'a str) -> Result<bool, ErrorAuditor> {
        let regex = match Regex::new(Self::APPLICATION_USER__EMAIL_REGULAR_EXPRESSION) {
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
            regex.is_match(application_user_email) && application_user_email.chars().count() <= Self::APPLICATION_USER__EMAIL_MAXIMUM_LENGTH
        );
    }

    pub fn is_valid_nickname<'a>(application_user_nickname: &'a str) -> bool {
        return application_user_nickname.chars().count() <= Self::APPLICATION_USER__NICKNAME_MAXIMUM_LENGTH
            && !application_user_nickname.contains('@')
            && !application_user_nickname.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user_nickname.is_empty();
    }

    pub fn is_valid_password<'a>(application_user_password: &'a str) -> bool {
        let password_chars_count = application_user_password.chars().count();

        return password_chars_count >= Self::APPLICATION_USER__PASSWORD_MINIMUM_LENGTH             // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
            && password_chars_count <= Self::APPLICATION_USER__PASSWORD_MAXIMUM_LENGTH
            && !application_user_password.contains(' ');
    }
}