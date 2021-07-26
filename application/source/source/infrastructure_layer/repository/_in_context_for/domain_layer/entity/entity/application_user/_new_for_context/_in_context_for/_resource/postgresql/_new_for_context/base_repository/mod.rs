use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::application_user::_core::id::Id;
use crate::domain_layer::entity::entity::application_user::_core::nickname::Nickname;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository_trait::BaseRepositoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::insert::Insert;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::select::Select;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::update::Update;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::infrastructure_layer::service::diesel_component::schema_descriptor::public::application_user as application_user_schema;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgConnection as Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl BaseRepositoryTrait for BaseRepository {
    fn create<'outer_a>(
        connection: &'outer_a Connection, application_user: &'outer_a ApplicationUser<'_>
    ) -> Result<(), BaseError> {
        diesel::insert_into(application_user_schema::table).values(Insert::new(application_user))
        .execute(connection)?;  // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a Connection, application_user: &'outer_a ApplicationUser<'_>, update_resolver: UpdateResolver
    ) -> Result<(), BaseError> {
        diesel::update(application_user_schema::table.filter(application_user_schema::id.eq(application_user.get_id()?.get_value())))
        .set(&Update::new(application_user, update_resolver)).execute(connection)?;

        return Ok(());
    }

    fn is_exist_by_nickanme<'outer_a>(connection: &'outer_a Connection, nickname: &'outer_a Nickname) -> Result<bool, BaseError> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::nickname.eq(nickname.get_value()))))
            .get_result::<bool>(connection)?
        );// TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    fn is_exist_by_email<'outer_a>(connection: &'outer_a Connection, email: &'outer_a Email) -> Result<bool, BaseError> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::email.eq(email.get_value()))))
            .get_result::<bool>(connection)?
        );      // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    fn get_by_email<'outer_a, 'vague>(connection: &'outer_a Connection, email: &'outer_a Email) -> Result<Option<ApplicationUser<'vague>>, BaseError> {
        if let Some(select) = application_user_schema::table.filter(application_user_schema::email.eq(email.get_value()))
        .get_result::<Select>(connection).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_select(select))); 
        }

        return Ok(None); 
    }

    fn get_by_id<'outer_a, 'vague>(connection: &'outer_a Connection, id: &'outer_a Id) -> Result<Option<ApplicationUser<'vague>>, BaseError> {
        if let Some(select) = application_user_schema::table.filter(application_user_schema::id.eq(id.get_value()))
        .get_result::<Select>(connection).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_select(select))); 
        }

        return Ok(None); 
    }
}

// TODO // TODO // TODO // TODO // TODO // TODO // TODO
// delete this after writing same query for another entity (Exampe of multyrow Select)
// pub fn get_by_application_user_id<'outer_a>(
//     connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a UuidV4
// ) -> Result<Option<Vec<JsonRefreshWebToken<'vague>>>, BaseError> {
//     let existing_registry = json_refresh_web_token_schema::table
//     .filter(json_refresh_web_token_schema::application_user_id.eq(application_user_id.get_value()))
//     .get_results::<Existing>(connection_manager.get_connection())?;
    
//     if !existing_registry.is_empty() {
//         return Ok(
//             Some(
//                 existing_registry.into_iter().map(
//                     |existing: Existing| -> JsonRefreshWebToken<'vague> { 
//                         return JsonRefreshWebToken::new_from_model(existing); 
//                     }
//                 ).collect::<Vec<JsonRefreshWebToken<'vague>>>()
//             )
//         ); 
//     }
    
//     return Ok(None); 
// }
// TODO // TODO // TODO // TODO // TODO // TODO // TODO