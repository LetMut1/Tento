use super::validator::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;

impl Validator<ApplicationUser_Nickname> {
    pub const MAXIMUM_LENGTH: usize = 55;

    pub fn is_valid<'a>(application_user_nickname: &'a ApplicationUser_Nickname) -> bool {
        let application_user_nickname_ = application_user_nickname.get();

        return application_user_nickname_.chars().count() <= Self::MAXIMUM_LENGTH
            && !application_user_nickname_.contains('@')
            && !application_user_nickname_.contains(' ')       // TODO Проверить символ табуляци TAB            НАПИСАТЬ Через Регекс?
            && !application_user_nickname_.is_empty();
    }
}
