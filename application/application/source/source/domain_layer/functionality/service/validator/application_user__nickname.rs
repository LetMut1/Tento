use super::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
impl Validator<ApplicationUser_Nickname> {
    pub fn is_valid<'a>(application_user_nickname: &'a str) -> bool {
        return application_user_nickname.chars().count() <= ApplicationUser_Nickname::MAXIMUM_LENGTH
            && !application_user_nickname.contains('@')
            && !application_user_nickname.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user_nickname.is_empty();
    }
}
