use crate::diesel_component::schema::public::application_user_registration_confirmation_token as application_user_registration_confirmation_token_schema;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::existing::Existing;
use crate::dto::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::new::New;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
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
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), DieselErrorKind> {
        match diesel::insert_into(application_user_registration_confirmation_token_schema::table).values(New::new(application_user_registration_confirmation_token))
        .execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn delete(
        connection_manager: &'outer ConnectionManager, 
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), DieselErrorKind> {
        match diesel::delete(
            application_user_registration_confirmation_token_schema::table
            .filter(application_user_registration_confirmation_token_schema::id.eq(application_user_registration_confirmation_token.get_id().get_value()))
        ).execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn update(
        connection_manager: &'outer ConnectionManager,
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), DieselErrorKind> {
        match diesel::update(
            application_user_registration_confirmation_token_schema::table
            .filter(application_user_registration_confirmation_token_schema::id.eq(application_user_registration_confirmation_token.get_id().get_value()))
        ).set(
            (
                application_user_registration_confirmation_token_schema::value.eq(application_user_registration_confirmation_token.get_value().get_value()),
                application_user_registration_confirmation_token_schema::expired_at.eq(application_user_registration_confirmation_token.get_expired_at().get_value())
            )
        ).execute(connection_manager.get_connection()) {
            Ok(_) => { return Ok(()); },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }

    pub fn get_by_pre_confirmed_application_user_id(
        connection_manager: &'outer ConnectionManager, pre_confirmed_application_user_id: &'outer UuidV4
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'vague>>, DieselErrorKind> {
        match application_user_registration_confirmation_token_schema::table.filter(
            application_user_registration_confirmation_token_schema::pre_confirmed_application_user_id.eq(pre_confirmed_application_user_id.get_value())
        ).get_result::<Existing>(connection_manager.get_connection()).optional() {
            Ok(existing) => { 
                match existing {
                    Some(existing) => { return Ok(Some(ApplicationUserRegistrationConfirmationToken::new_from_model(existing))); },
                    None => { return Ok(None); }
                };
             },
            Err(error) => { return Err(DieselErrorKind::new_any(error, None)); }
        };
    }
}