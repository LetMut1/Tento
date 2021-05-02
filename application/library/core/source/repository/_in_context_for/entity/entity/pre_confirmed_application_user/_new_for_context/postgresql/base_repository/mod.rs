use crate::diesel_component::schema::public::pre_confirmed_application_user as pre_confirmed_application_user_schema;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::existing::Existing;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::new::New;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer_a> BaseRepository {
    pub fn create(
        connection_manager: &'outer_a ConnectionManager, pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<(), ResourceErrorKind> {
        diesel::insert_into(pre_confirmed_application_user_schema::table).values(New::new(pre_confirmed_application_user))
        .execute(connection_manager.get_connection())?;   // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    pub fn delete(
        connection_manager: &'outer_a ConnectionManager, pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<(), ResourceErrorKind> {
        diesel::delete(
            pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::id.eq(pre_confirmed_application_user.get_id().get_value()))
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_email(connection_manager: &'outer_a ConnectionManager, email: &'outer_a Email) -> Result<bool, ResourceErrorKind> { // TODO сделать возможномть устанавливать фильтр ? 
        return Ok(
            diesel::select(dsl::exists(pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::email.eq(email.get_value()))))
            .get_result::<bool>(connection_manager.get_connection())?
        );          // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_email(connection_manager: &'outer_a ConnectionManager, email: &'outer_a Email) -> Result<Option<PreConfirmedApplicationUser>, ResourceErrorKind> {
        if let Some(existing) = pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::email.eq(email.get_value()))
        .get_result::<Existing>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(PreConfirmedApplicationUser::new_from_model(existing))); 
        }

        return Ok(None); 
    }
}