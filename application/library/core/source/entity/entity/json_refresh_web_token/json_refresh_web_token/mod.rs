use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::utility::_in_context_for::entity::entity::json_refresh_web_token::_new_context_for::date_expiration_creator::DateExpirationCreator;
use std::borrow::Cow;

pub struct JsonRefreshWebToken<'outer> {
    json_access_web_token_id: UuidV4,
    application_user_id: Cow<'outer, UuidV4>,
    application_user_log_in_token_device_id: Cow<'outer, UuidV4>,
    expired_at: DateTime
}

impl<'this, 'outer: 'this> JsonRefreshWebToken<'outer> {
    pub fn new(application_user_id: &'outer UuidV4, application_user_log_in_token_device_id: &'outer UuidV4) -> Self {
        return Self {
            json_access_web_token_id: UuidV4::new(),
            application_user_id: Cow::Borrowed(application_user_id),
            application_user_log_in_token_device_id: Cow::Borrowed(application_user_log_in_token_device_id),
            expired_at: DateExpirationCreator::create()
        };
    }

    pub fn new_from_model(existing: Existing) -> Self {
        return Self {
            json_access_web_token_id: UuidV4::new_from_uuid(existing.json_access_web_token_id),
            application_user_id: Cow::Owned(UuidV4::new_from_uuid(existing.application_user_id)),
            application_user_log_in_token_device_id: Cow::Owned(UuidV4::new_from_uuid(existing.application_user_log_in_token_device_id)),
            expired_at: DateTime::new_from_date_time(existing.expired_at)
        };
    }

    pub fn refresh(&'this mut self) -> &'this mut Self {
        return self.refresh_expired_at();
    }

    fn refresh_expired_at(&'this mut self) -> &'this mut Self {
        self.expired_at = DateExpirationCreator::create();

        return self;
    }

    pub fn get_json_access_web_token_id(&'this self) -> &'this UuidV4 {
        return &self.json_access_web_token_id;
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.application_user_id;
    }

    pub fn get_application_user_log_in_token_device_id(&'this self) -> &'this UuidV4 {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_expired_at(&'this self) -> &'this DateTime {
        return &self.expired_at;
    }
}