use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPostgresqlTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::base::Base as ApplicationUserFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::select::Select;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::diesel_component::schema_describer::public::application_user as application_user_schema;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgConnection as Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct Base;

impl DataProviderApplicationUserPostgresqlTrait for Base {
    type Error = BaseError;

    fn is_exist_by_nickanme<'outer_a>(
        connection: &'outer_a Connection,
        nickname: &'outer_a str
    ) -> Result<bool, Self::Error> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::nickname.eq(nickname))))
            .get_result::<bool>(connection)?
        );// TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    fn is_exist_by_email<'outer_a>(
        connection: &'outer_a Connection,
        email: &'outer_a str
    ) -> Result<bool, Self::Error> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::email.eq(email))))
            .get_result::<bool>(connection)?
        );      // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    fn get_by_email<'outer_a>(
        connection: &'outer_a Connection,
        email: &'outer_a str
    ) -> Result<Option<ApplicationUser<'static>>, Self::Error> {
        if let Some(select) = application_user_schema::table.filter(application_user_schema::email.eq(email))
        .get_result::<Select>(connection).optional()? 
        {
            return Ok(Some(ApplicationUserFactory::new_from_select(select))); 
        }

        return Ok(None); 
    }

    fn get_by_id<'outer_a>(
        connection: &'outer_a Connection,
        id: &'outer_a i64
    ) -> Result<Option<ApplicationUser<'static>>, Self::Error> {
        if let Some(select) = application_user_schema::table.filter(application_user_schema::id.eq(id))
        .get_result::<Select>(connection).optional()? 
        {
            return Ok(Some(ApplicationUserFactory::new_from_select(select))); 
        }

        return Ok(None); 
    }
}