use crate::domain_layer::entity::json_access_web_token::_component::payload::Payload;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use std::borrow::Cow;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn new_from_json_refresh_web_token<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<Payload<'outer_a>, Self::Error>;

    fn new_from_payload_common(
        payload_common: PayloadCommon<'static>
    ) -> Payload<'static> {
        let (
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            exp
        ) : (
            Cow<'_, str>,
            Cow<'_, i64>,
            Cow<'_, str>,
            Cow<'_, str>
        ) = payload_common.into_inner();

        return Payload::new(
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            exp.into_owned()
        )
    }
}