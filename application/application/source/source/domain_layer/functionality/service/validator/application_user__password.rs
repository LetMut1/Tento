use super::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
impl Validator<ApplicationUser_Password> {
    pub fn is_valid<'a>(
        application_user_password: &'a str,
        application_user_email: &'a str,
        application_user_nickname: &'a str,
    ) -> bool {
        return Self::is_valid_part_1(application_user_password)
            && Self::is_valid_part_2(
                application_user_password,
                application_user_email,
                application_user_nickname,
            );
    }
    pub fn is_valid_part_1<'a>(application_user_password: &'a str) -> bool {
        let password_chars_count = application_user_password.chars().count();
        return password_chars_count >= ApplicationUser_Password::MINIMUM_LENGTH
            && password_chars_count <= ApplicationUser_Password::MAXIMUM_LENGTH
            && !application_user_password.contains(' ');
    }
    pub fn is_valid_part_2<'a>(
        application_user_password: &'a str,
        application_user_email: &'a str,
        application_user_nickname: &'a str,
    ) -> bool {
        return application_user_password != application_user_email && application_user_password != application_user_nickname;
    }
}
