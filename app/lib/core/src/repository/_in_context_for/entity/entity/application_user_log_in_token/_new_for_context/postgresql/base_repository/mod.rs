use crate::diesel_component::schema::public::application_user_log_in_token as application_user_log_in_token_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer ConnectionManager, 
        application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::insert_into(application_user_log_in_token_schema::table)
        .values(New::new(application_user_log_in_token))
        .execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn delete(
        connection_manager: &'outer ConnectionManager, 
        application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::delete(
            application_user_log_in_token_schema::table
            .filter(application_user_log_in_token_schema::id.eq(application_user_log_in_token.get_id().get_value()))
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn update(
        connection_manager: &'outer ConnectionManager,
        application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::update(
            application_user_log_in_token_schema::table
            .filter(application_user_log_in_token_schema::id.eq(application_user_log_in_token.get_id().get_value()))
        ).set(
            (
                application_user_log_in_token_schema::value.eq(application_user_log_in_token.get_value().get_value()),
                application_user_log_in_token_schema::expired_at.eq(application_user_log_in_token.get_expired_at().get_value())
            )
        ).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_application_user_id(
        connection_manager: &'outer ConnectionManager, application_user_id: &'outer UuidV4
    ) -> Result<bool, DieselError> {
        return Ok(
            diesel::select(
                dsl::exists(application_user_log_in_token_schema::table.filter(application_user_log_in_token_schema::application_user_id.eq(application_user_id.get_value())))
            ).get_result::<bool>(connection_manager.get_connection())?
        ); // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_application_user_id_and_device_id(
        connection_manager: &'outer ConnectionManager, application_user_id: &'outer UuidV4, device_id: &'outer UuidV4,
    ) -> Result<Option<ApplicationUserLogInToken<'vague>>, DieselError> {
        match application_user_log_in_token_schema::table
        .filter(application_user_log_in_token_schema::application_user_id.eq(application_user_id.get_value()))
        .filter(application_user_log_in_token_schema::device_id.eq(device_id.get_value()))
        .get_result::<Existing>(connection_manager.get_connection()).optional()? {
            Some(existing) => { 
                return Ok(Some(ApplicationUserLogInToken::new_from_model(existing))); 
            },
            None => { 
                return Ok(None); 
            }
        };
    }
}