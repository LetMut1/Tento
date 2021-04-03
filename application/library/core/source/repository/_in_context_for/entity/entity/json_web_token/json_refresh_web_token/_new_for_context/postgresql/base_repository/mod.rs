use crate::diesel_component::schema::public::json_refresh_web_token as json_refresh_web_token_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
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
        connection_manager: &'outer ConnectionManager, json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>
    ) -> Result<(), DieselError> {
        diesel::insert_into(json_refresh_web_token_schema::table).values(New::new(json_refresh_web_token))
        .execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn get_by_application_user_id_and_application_user_log_in_token_device_id(
        connection_manager: &'outer ConnectionManager, 
        application_user_id: &'outer UuidV4, 
        application_user_log_in_token_device_id: &'outer UuidV4,
    ) -> Result<Option<JsonRefreshWebToken<'vague>>, DieselError> {
        if let Some(existing) = 
        json_refresh_web_token_schema::table
        .filter(json_refresh_web_token_schema::application_user_id.eq(application_user_id.get_value()))
        .filter(json_refresh_web_token_schema::application_user_log_in_token_device_id.eq(application_user_log_in_token_device_id.get_value()))
        .get_result::<Existing>(connection_manager.get_connection()).optional()?
        {
            return Ok(Some(JsonRefreshWebToken::new_from_model(existing))); 
        }

        return Ok(None); 
    }
}

// TODO При переходе на РЕдис:
// хранить рефреш-токен по ключу "апплтикэйшн-юзер-айди + логин-девайс_айди".
// для Выхода со всех устройств храниь все ДевайсыАди по ключу "АппликэйшнЮзерАйди"
// ставить срок экспирации кеша, равный сроку экспирации токена ( так же и в БлэкЛист)