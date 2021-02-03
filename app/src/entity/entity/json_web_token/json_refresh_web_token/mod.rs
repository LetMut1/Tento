pub mod core;

use crate::diesel_component::model::entity::entity::json_refresh_web_token::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::device_id::DeviceId;
use crate::entity::core::uuid_v4::UuidV4;
use crate::util::entity::entity::json_web_token::json_refresh_web_token::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;
use self::core::value::Value;

pub struct JsonRefreshWebToken<'a> {
    id: UuidV4<'a>,
    user_id:  MaybeOwned<'a, UuidV4<'a>>,
    device_id: DeviceId<'a>,
    value: Value<'a>,
    created_at: DateTime<'a>,
    expired_at: DateTime<'a>,
}

impl<'a> JsonRefreshWebToken<'a> {          // TODO  create ValHas with CustomHasher, value - это изменяемое после каждого использования токена поле. Может, завязать на device_id?
    pub fn new(user_id: &'a UuidV4, device_id: String, value: String) -> Self {
        return Self {
            id: UuidV4::new(),
            user_id: MaybeOwned::Borrowed(user_id),
            device_id: DeviceId::new(MaybeOwned::Owned(device_id)),
            value: Value::new(MaybeOwned::Owned(value)),
            created_at: DateTime::new(),
            expired_at: DateExpirationCreator::create_interval()
        };
    }

    pub fn new_from_model(model: &'a Existing) -> Self {
        return Self {
            id: UuidV4::new_from(MaybeOwned::Borrowed(model.get_id())),
            user_id: MaybeOwned::Owned(UuidV4::new_from(MaybeOwned::Borrowed(model.get_user_id()))),
            device_id: DeviceId::new(MaybeOwned::Borrowed(model.get_device_id())),
            value: Value::new(MaybeOwned::Borrowed(model.get_value_hash())),
            created_at: DateTime::new_from(MaybeOwned::Borrowed(model.get_created_at())),
            expired_at: DateTime::new_from(MaybeOwned::Borrowed(model.get_expired_at()))
        };
    }

    pub fn set_value(&'a mut self, value: String) -> &'a mut Self {
        self.value = Value::new(MaybeOwned::Owned(value));

        return self;
    }

    pub fn refresh_expired_at(&'a mut self) -> &'a mut Self {
        self.expired_at = DateExpirationCreator::create_interval();

        return self;
    }

    pub fn get_id(&'a self) -> &'a UuidV4 {
        return &self.id;
    }

    pub fn get_user_id(&'a self) -> &'a UuidV4 {
        return &self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a DeviceId {
        return &self.device_id;
    }

    pub fn get_value(&'a self) -> &'a Value {
        return &self.value;
    }

    pub fn get_created_at(&'a self) -> &'a DateTime {
        return &self.created_at;
    }

    pub fn get_expired_at(&'a self) -> &'a DateTime {
        return &self.expired_at;
    }
}