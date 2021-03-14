use crate::dto::resourse_model::_in_context_for::entity::entity::pre_registered_application_user_registration_confirmation_token::_new_for_context::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::pre_registered_application_user_registration_confirmation_token::core::value::Value;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;
use crate::utility::_in_context_for::entity::entity::pre_registered_apllication_user_registration_confirmation_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use std::borrow::Cow;

pub struct PreRegisteredApplicationUserRegistrationConfirmationToken<'outer> {
    id: UuidV4,
    pre_confirmed_application_user_id: Cow<'outer, UuidV4>,
    value: Value,
    expired_at: DateTime
}

impl<'this, 'outer: 'this> PreRegisteredApplicationUserRegistrationConfirmationToken<'outer> {
    pub fn new(pre_confirmed_application_user: &'outer PreConfirmedApplicationUser) -> Self {
        return Self {
            id: UuidV4::new(),
            pre_confirmed_application_user_id: Cow::Borrowed(pre_confirmed_application_user.get_id()),
            value: Value::new(UuidV4::new().get_value().to_string()),       // TODO какое значени лучше генерировать
            expired_at: DateExpirationCreator::create()
        };
    }

    pub fn new_from_model(existing: Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.id),
            pre_confirmed_application_user_id: Cow::Owned(UuidV4::new_from_uuid(existing.pre_confirmed_application_user_id)),
            value: Value::new(existing.value),
            expired_at: DateTime::new_from_date_time(existing.expired_at)
        };
    }

    pub fn is_expired(&'this self) -> bool {
        return !DateTimeManipulator::is_greate_or_equal_than_now(&self.expired_at);
    }

    pub fn get_id(&'this self) -> &'this UuidV4 {
        return &self.id;
    }

    pub fn get_pre_confirmed_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.pre_confirmed_application_user_id;
    }

    pub fn get_value(&'this self) -> &'this Value {
        return &self.value;
    }

    pub fn get_expired_at(&'this self) -> &'this DateTime {
        return &self.expired_at;
    }
}