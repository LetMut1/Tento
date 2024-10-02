use super::Validator;
use crate::domain_layer::data::entity::user::User_Nickname;
impl Validator<User_Nickname> {
    pub fn is_valid<'a>(application_user__nickname: &'a str) -> bool {
        return application_user__nickname.chars().count() <= User_Nickname::MAXIMUM_LENGTH
            && !application_user__nickname.contains('@')
            && !application_user__nickname.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user__nickname.is_empty();
    }
}
