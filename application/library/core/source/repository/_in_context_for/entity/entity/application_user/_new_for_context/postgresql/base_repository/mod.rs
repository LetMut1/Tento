use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::insert::Insert;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::select::Select;
use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::update::Update;
use crate::diesel_component::schema::public::application_user as application_user_schema;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::data_transfer_object::resource_model::_new_for_context::update_resolver::_in_context_for::_in_context_for::entity::entity::application_user::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager;
use diesel::dsl;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer ConnectionManager, application_user: &'outer ApplicationUser<'outer>
    ) -> Result<(), ResourceErrorKind> {
        diesel::insert_into(application_user_schema::table).values(Insert::new(application_user))
        .execute(connection_manager.get_connection())?;  // TODO нужно ли обработать количество вернувшихся строк

        return Ok(());
    }

    pub fn update(
        connection_manager: &'outer ConnectionManager, application_user: &'outer ApplicationUser<'outer>, update_resolver: UpdateResolver
    ) -> Result<(), ResourceErrorKind> {
        diesel::update(application_user_schema::table.filter(application_user_schema::id.eq(application_user.get_id().get_value())))
        .set(&Update::new(application_user, update_resolver)).execute(connection_manager.get_connection())?;

        return Ok(());
    }

    pub fn is_exist_by_nickanme(connection_manager: &'outer ConnectionManager, nickname: &'outer Nickname) -> Result<bool, ResourceErrorKind> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::nickname.eq(nickname.get_value()))))
            .get_result::<bool>(connection_manager.get_connection())?
        );// TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn is_exist_by_email(connection_manager: &'outer ConnectionManager, email: &'outer Email) -> Result<bool, ResourceErrorKind> {
        return Ok(
            diesel::select(dsl::exists(application_user_schema::table.filter(application_user_schema::email.eq(email.get_value()))))
            .get_result::<bool>(connection_manager.get_connection())?
        );      // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    pub fn get_by_email(connection_manager: &'outer ConnectionManager, email: &'outer Email) -> Result<Option<ApplicationUser<'vague>>, ResourceErrorKind> {
        if let Some(existing) = application_user_schema::table.filter(application_user_schema::email.eq(email.get_value()))
        .get_result::<Select>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_resource_model(existing))); 
        }

        return Ok(None); 
    }

    pub fn get_by_id(connection_manager: &'outer ConnectionManager, id: &'outer UuidV4) -> Result<Option<ApplicationUser<'vague>>, ResourceErrorKind> {
        if let Some(existing) = application_user_schema::table.filter(application_user_schema::id.eq(id.get_value()))
        .get_result::<Select>(connection_manager.get_connection()).optional()? 
        {
            return Ok(Some(ApplicationUser::new_from_resource_model(existing))); 
        }

        return Ok(None); 
    }
}