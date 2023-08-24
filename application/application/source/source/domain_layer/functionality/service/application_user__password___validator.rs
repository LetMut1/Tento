use super::validator::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;

impl Validator<ApplicationUser_Password> {
    pub fn is_valid<'a>(
        application_user_password: &'a ApplicationUser_Password,
        application_user_email: &'a ApplicationUser_Email,
        application_user_nickname: &'a ApplicationUser_Nickname,
    ) -> bool {
        return Self::is_valid_part_1(application_user_password)
            && Self::is_valid_part_2(
                application_user_password,
                application_user_email,
                application_user_nickname,
            );
    }

    pub fn is_valid_part_1<'a>(application_user_password: &'a ApplicationUser_Password) -> bool {
        let password_chars_count = application_user_password.0.chars().count();

        return password_chars_count >= ApplicationUser_Password::MINIMUM_LENGTH && password_chars_count <= ApplicationUser_Password::MAXIMUM_LENGTH && !application_user_password.0.contains(' ');
    }

    pub fn is_valid_part_2<'a>(
        application_user_password: &'a ApplicationUser_Password,
        application_user_email: &'a ApplicationUser_Email,
        application_user_nickname: &'a ApplicationUser_Nickname,
    ) -> bool {
        let application_user_password_ = application_user_password.0.as_str();

        return application_user_password_ != application_user_email.0.as_str() && application_user_password_ != application_user_nickname.0.as_str();
    }
}
