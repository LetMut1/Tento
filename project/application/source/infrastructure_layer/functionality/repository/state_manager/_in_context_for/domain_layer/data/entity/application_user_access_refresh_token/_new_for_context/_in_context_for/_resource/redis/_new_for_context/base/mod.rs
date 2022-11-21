use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::insert::Insert;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn create<'a, 'b>(
        connection: &'a mut Connection,
        insert: Insert<'b>
    ) -> Result<ApplicationUserAccessRefreshToken<'b>, ErrorAuditor> {
        todo!();
    }

    pub async fn update<'a>(
        connection: &'a mut Connection,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<(), ErrorAuditor> {
        todo!();
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<(), ErrorAuditor> {
        todo!();
    }
}