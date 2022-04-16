use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_encoder_trait::PasswordEncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use regex::Regex;

pub trait BaseTrait {
    type PasswordHashResolver: PasswordHashResolverTrait;

    const EMAIL_MAXIMUM_LENGTH: u16 = 320;
    const NICKNAME_MAXIMUM_LENGTH: u8 = 55;
    const PASSWORD_MINIMUM_LENGTH: u8 = 7;
    const PASSWORD_MAXIMUM_LENGTH: u8 = 65;

    fn is_valid_email<'a>(          // TODO Перенести реализацию в инфраструктуру
        email: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match Regex::new(r"\S+@\S+") {
            Ok(regex) => {
                return Ok(regex.is_match(email) && email.chars().count() <= (Self::EMAIL_MAXIMUM_LENGTH as usize))
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    fn is_valid_nickname<'a>(   // TODO Перенести реализацию в инфраструктуру
        nickname: &'a str
    ) -> bool {
        return nickname.chars().count() <= (Self::NICKNAME_MAXIMUM_LENGTH as usize)
            && !nickname.contains('@')
            && !nickname.contains(' ')     // TODO Проверить символ табуляци TAB
            && !nickname.is_empty();
    }

    fn is_valid_password<'a>(       // TODO Перенести реализацию в инфраструктуру
        password: &'a str
    ) -> bool {
        let password_chars_count = password.chars().count();

        return password_chars_count >= (Self::PASSWORD_MINIMUM_LENGTH as usize)
            && password_chars_count <= (Self::PASSWORD_MAXIMUM_LENGTH as usize)
            && !password.contains(' ');
    }   // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)

    fn is_valid_password_hash<'a>( // TODO убрать вообще
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, <<Self::PasswordHashResolver as PasswordHashResolverTrait>::PasswordEncoder as PasswordEncoderTrait>::Error> {
        return <Self::PasswordHashResolver as PasswordHashResolverTrait>::is_valid(password, password_hash);
    }
}