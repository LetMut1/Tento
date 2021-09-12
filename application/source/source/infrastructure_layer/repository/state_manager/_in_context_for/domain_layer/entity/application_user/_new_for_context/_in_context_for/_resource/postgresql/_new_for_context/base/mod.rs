use crate::domain_layer::entity::application_user::application_user::ApplicationUser;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserPostgresqlTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::insert::Insert;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::update::Update;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::infrastructure_layer::service::diesel_component::schema_describer::public::application_user as application_user_schema;
use diesel::ExpressionMethods;
use diesel::PgConnection as Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct Base;

impl StateManagerApplicationUserPostgresqlTrait for Base {
    type Error = BaseError;

    fn create<'outer_a>(
        connection: &'outer_a Connection,
        application_user: &'outer_a ApplicationUser<'_>
    ) -> Result<(), Self::Error> {
        diesel::insert_into(application_user_schema::table).values(Insert::new(application_user))
        .execute(connection)?;  // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a Connection,
        application_user: &'outer_a ApplicationUser<'_>,
        update_resolver: UpdateResolver
    ) -> Result<(), Self::Error> {
        diesel::update(application_user_schema::table.filter(application_user_schema::id.eq(application_user.get_id()?)))
        .set(&Update::new(application_user, update_resolver)).execute(connection)?;

        return Ok(());
    }
}