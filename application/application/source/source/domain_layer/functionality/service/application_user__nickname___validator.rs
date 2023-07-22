use super::validator::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;

impl Validator<ApplicationUser_Nickname> {
    pub fn is_valid<'a>(application_user_nickname: &'a ApplicationUser_Nickname) -> bool {
        return application_user_nickname.0.chars().count() <= ApplicationUser_Nickname::MAXIMUM_LENGTH
            && !application_user_nickname.0.contains('@')
            && !application_user_nickname.0.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user_nickname.0.is_empty();
    }
}
