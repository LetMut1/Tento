use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;

pub trait BaseTrait {
    fn create(
        id: Option<i64>,
        application_user_email: String,
        created_at: String
    ) -> ApplicationUserPreConfirmed {
        return ApplicationUserPreConfirmed::new(id, application_user_email, created_at);
    }

    fn create_from_application_user_email(
        application_user_email: String
    ) -> ApplicationUserPreConfirmed {
        return ApplicationUserPreConfirmed::new(
            None,
            application_user_email,
            crate::chrono::Utc::now().to_rfc2822()   // TODO 
        );
    }
}