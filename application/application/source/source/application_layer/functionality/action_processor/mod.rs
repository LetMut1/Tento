mod channel;
mod channel_subscription;
mod user_authorization;
pub use self::{
    channel::{
        check_linked_name_for_existing::Channel_CheckLinkedNameForExisting,
        check_name_for_existing::Channel_CheckNameForExisting,
        create::Channel_Create,
        get_many_by_name_in_subscriptions::Channel_GetManyByNameInSubscriptions,
        get_many_by_subscription::Channel_GetManyBySubscription,
        get_many_public_by_name::Channel_GetManyPublicByName,
        get_one_by_id::Channel_GetOneById,
    },
    channel_subscription::create::ChannelSubscription_Create,
    user_authorization::{
        authorize_by_first_step::UserAuthorization_AuthorizeByFirstStep,
        authorize_by_last_step::UserAuthorization_AuthorizeByLastStep,
        check_email_for_existing::UserAuthorization_CheckEmailForExisting,
        check_nickname_for_existing::UserAuthorization_CheckNicknameForExisting,
        deauthorize_from_all_devices::UserAuthorization_DeauthorizeFromAllDevices,
        deauthorize_from_one_device::UserAuthorization_DeauthorizeFromOneDevice,
        refresh_access_token::UserAuthorization_RefreshAccessToken,
        register_by_first_step::UserAuthorization_RegisterByFirstStep,
        register_by_last_step::UserAuthorization_RegisterByLastStep,
        register_by_second_step::UserAuthorization_RegisterBySecondStep,
        reset_password_by_first_step::UserAuthorization_ResetPasswordByFirstStep,
        reset_password_by_last_step::UserAuthorization_ResetPasswordByLastStep,
        reset_password_by_second_step::UserAuthorization_ResetPasswordBySecondStep,
        send_email_for_authorize::UserAuthorization_SendEmailForAuthorize,
        send_email_for_register::UserAuthorization_SendEmailForRegister,
        send_email_for_reset_password::UserAuthorization_SendEmailForResetPassword,
    },
};
use crate::infrastructure_layer::{
    data::{
        aggregate_error::AggregateError,
        capture::Capture,
        environment_configuration::EnvironmentConfiguration,
    },
    functionality::service::creator::PostgresqlConnectionPool,
};
use dedicated_crate::{
    unified_report::UnifiedReport,
    void::Void,
};
use std::{
    future::Future,
    marker::PhantomData,
};
use super::command_processor::RunServer;
pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}
pub trait ActionProcessor_ {
    type Incoming;
    type Outcoming;
    type Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>;
}
pub struct Inner<'a> {
    pub environment_configuration: &'static EnvironmentConfiguration<RunServer>,
    pub postgresql_connection_pool_database_1: &'a PostgresqlConnectionPool,
    pub postgresql_connection_pool_database_2: &'a PostgresqlConnectionPool,
}
