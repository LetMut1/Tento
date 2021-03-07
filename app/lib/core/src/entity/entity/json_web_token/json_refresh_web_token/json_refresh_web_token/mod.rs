use crate::diesel_component::model::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::device_id::DeviceId;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::utility::entity::entity::json_web_token::json_refresh_web_token::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;
use uuid::Uuid;

pub struct JsonRefreshWebToken<'this, 'outer> {
    id: UuidV4<'outer>,
    user_id:  MaybeOwned<'outer, UuidV4<'outer>>,
    device_id: DeviceId<'outer>,
    value: Value<'outer>,
    created_at: DateTime<'this>,    // TODO нужно ли это поле 
    expired_at: DateTime<'this>
}

impl<'this, 'outer: 'this> JsonRefreshWebToken<'this, 'outer> {    // TODO Redis disc      // TODO  create ValHas with CustomHasher, value - это изменяемое после каждого использования токена поле. Может, завязать на device_id?
    pub fn new(application_user: &'outer ApplicationUser<'outer>, device_id: &'outer String) -> Self {     // TODO Value генерировать внутри
        return Self {
            id: UuidV4::new(),
            user_id: MaybeOwned::Borrowed(application_user.get_id()),
            device_id: DeviceId::new(MaybeOwned::Borrowed(device_id)),
            value: Value::new(MaybeOwned::Owned(Uuid::new_v4().to_string())),
            created_at: DateTime::new(),
            expired_at: DateExpirationCreator::create_interval()
        };
    }

    pub fn new_from_model(existing: &'outer Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.get_id()),
            user_id: MaybeOwned::Owned(UuidV4::new_from_uuid(existing.get_user_id())),
            device_id: DeviceId::new(MaybeOwned::Borrowed(existing.get_device_id())),
            value: Value::new(MaybeOwned::Borrowed(existing.get_value_hash())),
            created_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(existing.get_created_at())),
            expired_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(existing.get_expired_at()))
        };
    }

    pub fn refresh_expired_at(&'this mut self) -> &'this mut Self {
        self.expired_at = DateExpirationCreator::create_interval();

        return self;
    }

    pub fn set_value(&'this mut self, value: Value<'outer>) -> &'this mut Self {
        self.value = value;

        return self;
    }

    pub fn get_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.id;
    }

    pub fn get_user_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.user_id;
    }

    pub fn get_device_id(&'this self) -> &'this DeviceId<'outer> {
        return &self.device_id;
    }

    pub fn get_value(&'this self) -> &'this Value<'outer> {
        return &self.value;
    }

    pub fn get_created_at(&'this self) -> &'this DateTime<'this> {
        return &self.created_at;
    }

    pub fn get_expired_at(&'this self) -> &'this DateTime<'this> {
        return &self.expired_at;
    }
}