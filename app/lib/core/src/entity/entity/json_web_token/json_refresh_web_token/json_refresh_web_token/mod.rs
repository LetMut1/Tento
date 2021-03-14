use crate::dto::resourse_model::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::application_user::ApplicationUser;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::device_id::DeviceId;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_context_for::date_expiration_creator::DateExpirationCreator;
use std::borrow::Cow;
use uuid::Uuid;

pub struct JsonRefreshWebToken<'outer> {
    id: UuidV4,
    application_user_id: Cow<'outer, UuidV4>,
    device_id: DeviceId,
    value: Value,
    created_at: DateTime,    // TODO нужно ли это поле 
    expired_at: DateTime
}

impl<'this, 'outer: 'this> JsonRefreshWebToken<'outer> {    // TODO Redis disc      // TODO  create ValHas with CustomHasher, value - это изменяемое после каждого использования токена поле. Может, завязать на device_id?
    pub fn new(application_user: &'outer ApplicationUser<'outer>, device_id: DeviceId) -> Self {
        return Self {
            id: UuidV4::new(),
            application_user_id: Cow::Borrowed(application_user.get_id()),
            device_id,
            value: Value::new(Uuid::new_v4().to_string()),
            created_at: DateTime::new(),
            expired_at: DateExpirationCreator::create()
        };
    }

    pub fn new_from_model(existing: Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.id),
            application_user_id: Cow::Owned(UuidV4::new_from_uuid(existing.application_user_id)),
            device_id: DeviceId::new(existing.device_id),
            value: Value::new(existing.value),
            created_at: DateTime::new_from_date_time(existing.created_at),
            expired_at: DateTime::new_from_date_time(existing.expired_at)
        };
    }

    pub fn refresh_expired_at(&'this mut self) -> &'this mut Self {
        self.expired_at = DateExpirationCreator::create();

        return self;
    }

    pub fn set_value(&'this mut self, value: Value) -> &'this mut Self {
        self.value = value;

        return self;
    }

    pub fn get_id(&'this self) -> &'this UuidV4 {
        return &self.id;
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.application_user_id;
    }

    pub fn get_device_id(&'this self) -> &'this DeviceId {
        return &self.device_id;
    }

    pub fn get_value(&'this self) -> &'this Value {
        return &self.value;
    }

    pub fn get_created_at(&'this self) -> &'this DateTime {
        return &self.created_at;
    }

    pub fn get_expired_at(&'this self) -> &'this DateTime {
        return &self.expired_at;
    }
}