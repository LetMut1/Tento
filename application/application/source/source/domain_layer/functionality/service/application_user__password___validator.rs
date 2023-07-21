use super::validator::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;

impl Validator<ApplicationUser_Password> {
    const MINIMUM_LENGTH: usize = 7;
    const MAXIMUM_LENGTH: usize = 65;   // TODO Нужна ли максимальная длина? // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)

    pub fn is_valid<'a>(
        application_user_password: &'a ApplicationUser_Password,
        application_user_email: &'a ApplicationUser_Email,
        application_user_nickname: &'a ApplicationUser_Nickname,
    ) -> bool {
        return Self::is_valid_part_1(application_user_password)
            && Self::is_valid_part_2(application_user_password, application_user_email, application_user_nickname);
    }

    pub fn is_valid_part_1<'a>(
        application_user_password: &'a ApplicationUser_Password,
    ) -> bool {
        let password_chars_count = application_user_password.0.chars().count();

        return password_chars_count >= Self::MINIMUM_LENGTH
            && password_chars_count <= Self::MAXIMUM_LENGTH
            && !application_user_password.0.contains(' ');
    }

    pub fn is_valid_part_2<'a>(
        application_user_password: &'a ApplicationUser_Password,
        application_user_email: &'a ApplicationUser_Email,
        application_user_nickname: &'a ApplicationUser_Nickname,
    ) -> bool {
        let application_user_password_ = application_user_password.0.as_str();

        return application_user_password_ != application_user_email.0.as_str()
            && application_user_password_ != application_user_nickname.0.as_str();
    }
}
