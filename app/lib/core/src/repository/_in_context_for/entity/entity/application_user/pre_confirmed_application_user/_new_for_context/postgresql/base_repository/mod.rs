use crate::diesel_component::schema::public::pre_confirmed_application_user as pre_confirmed_application_user_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::pre_confirmed_application_user::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::pre_confirmed_application_user::_new_for_context::new::New;
use crate::entity::entity::application_user::application_user::core::email::Email;
use crate::entity::entity::application_user::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, pre_confirmed_application_user: &'outer PreConfirmedApplicationUser) -> Result<(), DieselError> {
        diesel::insert_into(pre_confirmed_application_user_schema::table).values(New::new(pre_confirmed_application_user))
        .execute(connection_manager.get_connection())?;   // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    pub fn delete(connection_manager: &'outer ConnectionManager, pre_confirmed_application_user: &'outer PreConfirmedApplicationUser) -> Result<(), DieselError> {
        diesel::delete(
            pre_confirmed_application_user_schema::table
            .filter(pre_confirmed_application_user_schema::id.eq(pre_confirmed_application_user.get_id().get_value()))
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_email(connection_manager: &'outer ConnectionManager, email: &'outer Email) -> Result<bool, DieselError> { // TODO сделать возможномть устанавливать фильтр ? 
        return Ok(diesel::select(
            dsl::exists(
                pre_confirmed_application_user_schema::table
                .filter(pre_confirmed_application_user_schema::email.eq(email.get_value()))
            )
        ).get_result::<bool>(connection_manager.get_connection())?
        );          // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_email(connection_manager: &'outer ConnectionManager, email: &'outer Email) -> Result<Option<PreConfirmedApplicationUser>, DieselError> {
        match pre_confirmed_application_user_schema::table.filter(pre_confirmed_application_user_schema::email.eq(email.get_value()))
        .get_result::<Existing>(connection_manager.get_connection()).optional()? {
            Some(existing) => { 
                return Ok(Some(PreConfirmedApplicationUser::new_from_model(existing))); 
            },
            None => { 
                return Ok(None); 
            }
        };
    }
}