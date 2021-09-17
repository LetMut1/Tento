use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user::ApplicationUser;
use std::borrow::Cow;

pub trait BaseTrait {
    fn new_from_application_user_pre_confirmed<'outer_a>(
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed,
        nickname: String,
        password_hash: String
    ) -> ApplicationUser<'_> {
        return ApplicationUser::new(
            None,
            Cow::Borrowed(application_user_pre_confirmed.get_application_user_email()),
            nickname,
            password_hash,
            chrono::Utc::now().to_rfc2822() // TODO 
        );
    }
}