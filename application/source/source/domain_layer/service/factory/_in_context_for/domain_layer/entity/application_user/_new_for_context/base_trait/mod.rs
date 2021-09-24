use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user::ApplicationUser;

pub trait BaseTrait {
    fn new_from_application_user_pre_confirmed(
        application_user_pre_confirmed: ApplicationUserPreConfirmed,
        nickname: String,
        password_hash: String
    ) -> ApplicationUser {
        return ApplicationUser::new(
            None,
            application_user_pre_confirmed.get_application_user_email_as_owner(),
            nickname,
            password_hash,
            chrono::Utc::now().to_rfc2822() // TODO 
        );
    }
}