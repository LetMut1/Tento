use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessRefreshToken_PropertyGenerator;

impl ApplicationUserAccessRefreshToken_PropertyGenerator {

    pub fn generate_updated_at() -> i64 {
        return DateTimeResolver::unixtime_get_now();
    }
}