use crate::diesel_component::schema::public::json_access_web_token_black_list as json_access_web_token_black_list_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
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
        connection_manager: &'outer ConnectionManager, json_access_web_token_black_list: &'outer JsonAccessWebTokenBlackList<'outer>
    ) -> Result<(), DieselError> {
        diesel::insert_into(json_access_web_token_black_list_schema::table).values(New::new(json_access_web_token_black_list))
        .execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_json_refresh_token_id(connection_manager: &'outer ConnectionManager, json_refresh_web_token_id: &'outer UuidV4) -> Result<bool, DieselError> {
        return Ok(
            diesel::select(
                dsl::exists(
                    json_access_web_token_black_list_schema::table
                    .filter(json_access_web_token_black_list_schema::json_refresh_web_token_id.eq(json_refresh_web_token_id.get_value()))
                )
            )
            .get_result::<bool>(connection_manager.get_connection())?
        );  // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }
}