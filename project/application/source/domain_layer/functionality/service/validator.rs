use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::Id as ApplicationUserId;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::regex::Regex;
use std::marker::PhantomData;

pub struct Validator<E, F> {
    _entity: PhantomData<E>,
    _field: PhantomData<F>
}

impl Validator<ApplicationUser<'_>, ApplicationUserId> {
    pub fn is_valid<'a>(application_user_id: i64) -> bool {
        return application_user_id >= 0;
    }
}