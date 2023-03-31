use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessToken_PropertyGenerator;

impl ApplicationUserAccessToken_PropertyGenerator {
    pub fn generate_id() -> String {
        return Uuid::new_v4().to_string();
    }
}