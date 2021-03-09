use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_registration_confirmation_token::core::value::Value;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::existing::Existing;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use crate::utility::_in_context_for::entity::entity::apllication_user_registration_confirmation_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;

pub struct ApplicationUserRegistrationConfirmationToken<'outer> {
    id: UuidV4<'outer>,
    application_user_id: UuidV4<'outer>,
    value: Value<'outer>,
    expired_at: DateTime<'outer>
}

impl<'this, 'outer: 'this> ApplicationUserRegistrationConfirmationToken<'outer> {
    pub fn new(application_user: &'outer ApplicationUser<'outer>) -> Self {
        return Self {
            id: UuidV4::new(),
            application_user_id: UuidV4::new_from_uuid(application_user.get_id().get_value()),
            value: Value::new(MaybeOwned::Owned(UuidV4::new().get_value().to_string())),    // TODO какое значени лучше генерировать
            expired_at: DateExpirationCreator::create()
        };
    }

    pub fn new_from_model(existing: &'outer Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.get_id()),
            application_user_id: UuidV4::new_from_uuid(existing.get_application_user_id()),
            value: Value::new(MaybeOwned::Borrowed(existing.get_value())),
            expired_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(existing.get_created_at()))
        };
    }

    pub fn is_expired(&'this self) -> bool {
        return !DateTimeManipulator::is_greate_or_equal_than_now(&self.expired_at);
    }

    pub fn get_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.id;
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.application_user_id;
    }

    pub fn get_value(&'this self) -> &'this Value<'outer> {
        return &self.value;
    }

    pub fn get_expired_at(&'this self) -> &'this DateTime<'outer> {
        return &self.expired_at;
    }
}