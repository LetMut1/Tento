use crate::infrastructure_layer::diesel_component::schema::public::pre_confirmed_application_user as pre_confirmed_application_user_schema;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::select::Select;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::insert::Insert;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::domain_layer::error::base_error::base_error::BaseError;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgConnection as Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl BaseRepository {
    pub fn create<'outer_a>(
        connection: &'outer_a Connection, pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<(), BaseError> {
        diesel::insert_into(pre_confirmed_application_user_schema::table).values(Insert::new(pre_confirmed_application_user))
        .execute(connection)?;   // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    pub fn delete<'outer_a>(
        connection: &'outer_a Connection, pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<(), BaseError> {
        diesel::delete(
            pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::id.eq(
                pre_confirmed_application_user.get_id()?.get_value()
            ))
        ).execute(connection)?;

        return Ok(());
    }

    pub fn is_exist_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection, application_user_email: &'outer_a Email
    ) -> Result<bool, BaseError> {
        return Ok(
            diesel::select(dsl::exists(pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::email.eq(application_user_email.get_value()))))
            .get_result::<bool>(connection)?
        );          // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection, application_user_email: &'outer_a Email
    ) -> Result<Option<PreConfirmedApplicationUser>, BaseError> {
        if let Some(select) = pre_confirmed_application_user_schema::table.filter(
            pre_confirmed_application_user_schema::email.eq(application_user_email.get_value())
        ).get_result::<Select>(connection).optional()? 
        {
            return Ok(Some(PreConfirmedApplicationUser::new_from_select(select))); 
        }

        return Ok(None); 
    }
}