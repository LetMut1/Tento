use super::Validator;
use crate::domain_layer::data::entity::user::User_Nickname;
impl Validator<User_Nickname> {
    pub fn is_valid<'a>(user__nickname: &'a str) -> bool {
        return user__nickname.chars().count() <= User_Nickname::MAXIMUM_LENGTH
            && !user__nickname.contains('@')
            && !user__nickname.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !user__nickname.is_empty();
    }
}
