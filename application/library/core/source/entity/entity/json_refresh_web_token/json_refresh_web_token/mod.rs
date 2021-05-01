use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use crate::utility::date_time_expiration_creator::DateTimeExpirationCreator;
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
            expired_at: DateTimeExpirationCreator::create_json_refresh_web_token_first()
        };
    }

    pub fn new_from_model(common: Common) -> Result<Self, InvalidArgumentError> {
        return Ok(
            Self {
                json_access_web_token_id: UuidV4::new_from_string(common.json_access_web_token_id)?,
                application_user_id: Cow::Owned(UuidV4::new_from_string(common.application_user_id)?),
                application_user_log_in_token_device_id: Cow::Owned(UuidV4::new_from_string(common.application_user_log_in_token_device_id)?),
                expired_at: DateTime::new_from_str(common.expired_at.as_str())
            }
        );
    }

    pub fn refresh(&'this mut self) -> &'this mut Self {
        return self.refresh_expired_at();
    }

    fn refresh_expired_at(&'this mut self) -> &'this mut Self {
        self.expired_at = DateTimeExpirationCreator::create_json_refresh_web_token_first();

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