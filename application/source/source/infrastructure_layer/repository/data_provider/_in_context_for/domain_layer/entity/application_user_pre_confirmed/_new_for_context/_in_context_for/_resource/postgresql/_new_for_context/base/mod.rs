use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPreConfirmedPostgesqlTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::select::Select;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::diesel_component::schema_describer::public::pre_confirmed_application_user as pre_confirmed_application_user_schema;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::base::Base as ApplicationUserPreConfirmedFactory;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgConnection as Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct Base;

impl DataProviderApplicationUserPreConfirmedPostgesqlTrait for Base {
    type Error = BaseError;

    fn is_exist_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection,
        application_user_email: &'outer_a str
    ) -> Result<bool, Self::Error> {
        return Ok(
            diesel::select(dsl::exists(pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::email.eq(application_user_email))))
            .get_result::<bool>(connection)?
        );          // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    fn get_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection,
        application_user_email: &'outer_a str
    ) -> Result<Option<ApplicationUserPreConfirmed>, Self::Error> {
        if let Some(select) = pre_confirmed_application_user_schema::table.filter(
            pre_confirmed_application_user_schema::email.eq(application_user_email)
        ).get_result::<Select>(connection).optional()? 
        {
            return Ok(Some(ApplicationUserPreConfirmedFactory::new_from_select(select))); 
        }

        return Ok(None); 
    }
}