use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;

pub trait BaseTrait {
    fn new_from_email(
        email: String
    ) -> ApplicationUserPreConfirmed {
        return ApplicationUserPreConfirmed::new(
            None,
            email,
            crate::chrono::Utc::now().to_rfc2822()   // TODO 
        );
    }
}