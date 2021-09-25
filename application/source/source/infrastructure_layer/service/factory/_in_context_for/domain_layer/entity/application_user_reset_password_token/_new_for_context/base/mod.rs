use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::base_trait::BaseTrait as ApplicationUserResetPasswordTokenFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::value_generator::ValueGenerator;
use std::borrow::Cow;

pub struct Base;

impl Base {
    pub fn create_from_common<'outer_a>(
        common: Common<'outer_a>,
        application_user_id: &'outer_a i64
    ) -> ApplicationUserResetPasswordToken<'outer_a> {
        let (
            application_user_email,
            value,
            wrong_enter_tries_quantity
        ) : (
            Cow<'_, str>,
            Cow<'_, str>,
            Cow<'_, u8>
        ) = common.into_inner();

        return ApplicationUserResetPasswordToken::new(
            application_user_id,
            application_user_email,
            value.into_owned(),
            wrong_enter_tries_quantity.into_owned()
        );
    }
}

impl ApplicationUserResetPasswordTokenFactoryTrait for Base {
    type Error = BaseError;
    type ValueGenerator = ValueGenerator;
}