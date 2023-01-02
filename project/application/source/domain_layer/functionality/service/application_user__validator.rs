use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use extern_crate::regex::Regex;

#[allow(non_camel_case_types)]
pub struct ApplicationUser_Validator;

impl ApplicationUser_Validator {
    #[allow(non_upper_case_globals)]
    const APPLICATION_USER__EMAIL_MAXIMUM_LENGTH: usize = 320;
    #[allow(non_upper_case_globals)]
    const APPLICATION_USER__NICKNAME_MAXIMUM_LENGTH: usize = 55;
    #[allow(non_upper_case_globals)]
    const APPLICATION_USER__PASSWORD_MINIMUM_LENGTH: usize = 7;
    #[allow(non_upper_case_globals)]
    const APPLICATION_USER__PASSWORD_MAXIMUM_LENGTH: usize = 65;

    pub fn is_valid_email<'a>(application_user_email: &'a str) -> Result<bool, ErrorAuditor> {
        match Regex::new(r"\S+@\S+") {
            Ok(regex) => {
                return Ok(
                    regex.is_match(application_user_email)
                        && application_user_email.chars().count() <= Self::APPLICATION_USER__EMAIL_MAXIMUM_LENGTH
                );
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
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