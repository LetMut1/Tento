use crate::diesel_component::schema::public::application_user as application_user_schema;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::select::Select;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::insert::Insert;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::core::email::Email;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(connection_manager: &'outer ConnectionManager, application_user: &'outer ApplicationUser<'outer>) -> Result<(), DieselError> {
        diesel::insert_into(application_user_schema::table).values(Insert::new(application_user)).execute(connection_manager.get_connection())?;  // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    pub fn update_password(
        connection_manager: &'outer ConnectionManager,
        application_user: &'outer ApplicationUser<'outer>
    ) -> Result<(), DieselError> {
        diesel::update(
            application_user_schema::table
            .filter(application_user_schema::id.eq(application_user.get_id().get_value()))
        ).set(
            application_user_schema::password_hash.eq(application_user.get_passord_hash().get_value())
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_nickanme(connection_manager: &'outer ConnectionManager, nickname: &'outer Nickname) -> Result<bool, DieselError> {
        return Ok(
            diesel::select(
                dsl::exists(
                    application_user_schema::table
                    .filter(application_user_schema::nickname.eq(nickname.get_value()))
                )
            )
            .get_result::<bool>(connection_manager.get_connection())?
        );// TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn is_exist_by_email(connection_manager: &'outer ConnectionManager, email: &'outer Email) -> Result<bool, DieselError> {
        return Ok(
            diesel::select(
                dsl::exists(
                    application_user_schema::table
                    .filter(application_user_schema::email.eq(email.get_value()))
                )
            )
            .get_result::<bool>(connection_manager.get_connection())?
        );      // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_email(connection_manager: &'outer ConnectionManager, email: &'outer Email) -> Result<Option<ApplicationUser<'vague>>, DieselError> {
        if let Some(existing) = application_user_schema::table
        .filter(application_user_schema::email.eq(email.get_value()))
        .get_result::<Select>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_resource_model(existing))); 
        }

        return Ok(None); 
    }

    pub fn get_by_id(connection_manager: &'outer ConnectionManager, id: &'outer UuidV4) -> Result<Option<ApplicationUser<'vague>>, DieselError> {
        if let Some(existing) = application_user_schema::table
        .filter(application_user_schema::id.eq(id.get_value()))
        .get_result::<Select>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_resource_model(existing))); 
        }

        return Ok(None); 
    }
}