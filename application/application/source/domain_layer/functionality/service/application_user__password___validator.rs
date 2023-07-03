use super::validator::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;

impl Validator<ApplicationUser_Password> {
    const MINIMUM_LENGTH: usize = 7;
    const MAXIMUM_LENGTH: usize = 65;

    pub fn is_valid<'a>(application_user_password: &'a ApplicationUser_Password) -> bool {
        let application_user_password_ = application_user_password.get();

        let password_chars_count = application_user_password_.chars().count();

        return password_chars_count >= Self::MINIMUM_LENGTH             // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
            && password_chars_count <= Self::MAXIMUM_LENGTH
            && !application_user_password_.contains(' ');
    }
}
