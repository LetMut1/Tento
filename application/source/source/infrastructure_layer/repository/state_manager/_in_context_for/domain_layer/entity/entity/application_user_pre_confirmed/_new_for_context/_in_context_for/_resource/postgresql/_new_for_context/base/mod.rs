use crate::domain_layer::entity::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserPreConfirmedPostgesqlTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::insert::Insert;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::diesel_component::schema_describer::public::pre_confirmed_application_user as pre_confirmed_application_user_schema;
use diesel::ExpressionMethods;
use diesel::PgConnection as Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct Base;

impl StateManagerApplicationUserPreConfirmedPostgesqlTrait for Base {
    fn create<'outer_a>(
        connection: &'outer_a Connection, application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), BaseError> {
        diesel::insert_into(pre_confirmed_application_user_schema::table).values(Insert::new(application_user_pre_confirmed))
        .execute(connection)?;   // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    fn delete<'outer_a>(
        connection: &'outer_a Connection, application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), BaseError> {
        diesel::delete(
            pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::id.eq(
                application_user_pre_confirmed.get_id()?.get_value()
            ))
        ).execute(connection)?;

        return Ok(());
    }
}