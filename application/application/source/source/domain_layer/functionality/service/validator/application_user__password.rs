use super::Validator;
use crate::domain_layer::data::entity::user::User_Password;
impl Validator<User_Password> {
    pub fn is_valid<'a>(application_user__password: &'a str, user__email: &'a str, application_user__nickname: &'a str) -> bool {
        return Self::is_valid_part_1(application_user__password)
            && Self::is_valid_part_2(
                application_user__password,
                user__email,
                application_user__nickname,
            );
    }
    pub fn is_valid_part_1<'a>(application_user__password: &'a str) -> bool {
        let password_chars_count = application_user__password.chars().count();
        return password_chars_count >= User_Password::MINIMUM_LENGTH
            && password_chars_count <= User_Password::MAXIMUM_LENGTH
            && !application_user__password.contains(' ');
    }
    pub fn is_valid_part_2<'a>(application_user__password: &'a str, user__email: &'a str, application_user__nickname: &'a str) -> bool {
        return application_user__password != user__email && application_user__password != application_user__nickname;
    }
}
