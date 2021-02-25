use crate::diesel_component::model::entity::entity::json_web_token::json_refresh_web_token::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::device_id::DeviceId;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::utility::entity::entity::json_web_token::json_refresh_web_token::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;
use uuid::Uuid;

pub struct JsonRefreshWebToken<'a, 'b> {
    id: UuidV4<'b>,
    user_id:  MaybeOwned<'b, UuidV4<'b>>,
    device_id: DeviceId<'b>,
    value: Value<'b>,
    created_at: DateTime<'a>,
    expired_at: DateTime<'a>
}

impl<'a, 'b: 'a> JsonRefreshWebToken<'a, 'b> {          // TODO  create ValHas with CustomHasher, value - это изменяемое после каждого использования токена поле. Может, завязать на device_id?
    pub fn new_from_credentials(user_id: MaybeOwned<'b, UuidV4<'b>>, device_id: &'b String) -> Self {     // TODO Value генерировать внутри
        return Self {
            id: UuidV4::new(),
            user_id,
            device_id: DeviceId::new(MaybeOwned::Borrowed(device_id)),
            value: Value::new(MaybeOwned::Owned(Uuid::new_v4().to_string())),
            created_at: DateTime::new(),
            expired_at: DateExpirationCreator::create_interval()
        };
    }

    pub fn new_from_model(existing: &'b Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.get_id()),
            user_id: MaybeOwned::Owned(UuidV4::new_from_uuid(existing.get_user_id())),
            device_id: DeviceId::new(MaybeOwned::Borrowed(existing.get_device_id())),
            value: Value::new(MaybeOwned::Borrowed(existing.get_value_hash())),
            created_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(existing.get_created_at())),
            expired_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(existing.get_expired_at()))
        };
    }

    pub fn refresh_expired_at(&'a mut self) -> &'a mut Self {
        self.expired_at = DateExpirationCreator::create_interval();

        return self;
    }

    pub fn set_value(&'a mut self, value: Value<'b>) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_id(&'a self) -> &'a UuidV4<'b> {
        return &self.id;
    }

    pub fn get_user_id(&'a self) -> &'a UuidV4<'b> {
        return &self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a DeviceId<'b> {
        return &self.device_id;
    }

    pub fn get_value(&'a self) -> &'a Value<'b> {
        return &self.value;
    }

    pub fn get_created_at(&'a self) -> &'a DateTime<'a> {
        return &self.created_at;
    }

    pub fn get_expired_at(&'a self) -> &'a DateTime<'a> {
        return &self.expired_at;
    }
}