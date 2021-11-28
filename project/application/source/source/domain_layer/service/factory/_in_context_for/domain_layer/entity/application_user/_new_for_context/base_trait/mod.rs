use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user::ApplicationUser;

pub trait BaseTrait {
    fn create(
        id: Option<i64>,
        email: String,
        nickname: String,
        password_hash: String,
        created_at: String
    ) -> ApplicationUser {
        return ApplicationUser::new(id, email, nickname, password_hash, created_at);
    }

    fn create_from_application_user_pre_confirmed(
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