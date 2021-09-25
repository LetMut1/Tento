use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::base_trait::BaseTrait as ApplicationUserLogInTokenFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator::ValueGenerator;
use std::borrow::Cow;

pub struct Base;

impl Base {
    pub fn create_from_common<'a>(
        common: Common<'a>,
        application_user_id: &'a i64,
        device_id: &'a str
    ) -> ApplicationUserLogInToken<'a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) : (
            Cow<'_, str>,
            Cow<'_, str>,
            Cow<'_, u8>
        ) = common.into_inner();

        return ApplicationUserLogInToken::new(
            application_user_id,
            device_id,
            application_user_email,
            value.into_owned(),
            wrong_enter_tries_quantity.into_owned()
        );
    }
}

impl ApplicationUserLogInTokenFactoryTrait for Base {
    type Error = BaseError;
    type ValueGenerator = ValueGenerator;
}