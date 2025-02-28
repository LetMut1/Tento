use {
    super::Validator,
    crate::domain_layer::data::entity::user::User_Password,
};
impl Validator<User_Password> {
    pub fn is_valid<'a>(user__password: &'a str, user__email: &'a str, user__nickname: &'a str) -> bool {
        return Self::is_valid_part_1(user__password)
            && Self::is_valid_part_2(
                user__password,
                user__email,
                user__nickname,
            );
    }
    pub fn is_valid_part_1<'a>(user__password: &'a str) -> bool {
        let password_chars_count = user__password.chars().count();
        return password_chars_count >= User_Password::MINIMUM_LENGTH && password_chars_count <= User_Password::MAXIMUM_LENGTH && !user__password.contains(' ');
    }
    pub fn is_valid_part_2<'a>(user__password: &'a str, user__email: &'a str, user__nickname: &'a str) -> bool {
        return user__password != user__email && user__password != user__nickname;
    }
}
