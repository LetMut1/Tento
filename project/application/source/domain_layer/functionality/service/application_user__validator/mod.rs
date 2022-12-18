use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use extern_crate::regex::Regex;

#[allow(non_camel_case_types)]
pub struct ApplicationUser_Validator;

impl ApplicationUser_Validator {
    const EMAIL_MAXIMUM_LENGTH: u16 = 320;
    const NICKNAME_MAXIMUM_LENGTH: u8 = 55;
    const PASSWORD_MINIMUM_LENGTH: u8 = 7;
    const PASSWORD_MAXIMUM_LENGTH: u8 = 65;

    pub fn is_valid_email<'a>(
        application_user_email: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match Regex::new(r"\S+@\S+") {
            Ok(regex) => {
                return Ok(
                    regex.is_match(application_user_email)
                        && application_user_email.chars().count() <= (Self::EMAIL_MAXIMUM_LENGTH as usize)
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

    pub fn is_valid_nickname<'a>(
        application_user_nickname: &'a str
    ) -> bool {
        return application_user_nickname.chars().count() <= (Self::NICKNAME_MAXIMUM_LENGTH as usize)
            && !application_user_nickname.contains('@')
            && !application_user_nickname.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user_nickname.is_empty();
    }

    pub fn is_valid_password<'a>(
        application_user_password: &'a str
    ) -> bool {
        let password_chars_count = application_user_password.chars().count();

        return password_chars_count >= (Self::PASSWORD_MINIMUM_LENGTH as usize)             // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
            && password_chars_count <= (Self::PASSWORD_MAXIMUM_LENGTH as usize)
            && !application_user_password.contains(' ');
    }
}