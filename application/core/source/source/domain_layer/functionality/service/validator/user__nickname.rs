use {
    super::Validator,
    crate::domain_layer::data::entity::user::User_Nickname,
};
impl Validator<User_Nickname> {
    pub fn is_valid<'a>(user__nickname: &'a str) -> bool {
        let chars = user__nickname.chars();
        if user__nickname.chars().count() > User_Nickname::MAXIMUM_LENGTH {
            return false;
        }
        '_a: for r#char in chars {
            if r#char.is_uppercase() {
                return false;
            }
        }
        return !user__nickname.contains('@') && !user__nickname.contains(' ') && !user__nickname.is_empty();      // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
    }
}
