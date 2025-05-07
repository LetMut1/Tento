mod channel;
mod channel_publication1;
mod channel_publication1_commentary;
mod channel_publication1_mark;
mod channel_publication1_view;
mod channel_subscription;
mod user;
use {
    super::command_processor::RunServer,
    crate::infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::creator::PostgresqlConnectionPool,
    },
    dedicated::unified_report::UnifiedReport,
    std::{
        future::Future,
        marker::PhantomData,
    },
};
pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}
pub trait ActionProcessor_ {
    type Incoming<'a>;
    type Outcoming;
    type Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send;
}
pub struct Inner<'a> {
    pub environment_configuration: &'static EnvironmentConfiguration<RunServer>,
    pub postgresql_connection_pool_database_1: &'a PostgresqlConnectionPool,
    pub postgresql_connection_pool_database_2: &'a PostgresqlConnectionPool,
    pub postgresql_connection_pool_database_3: &'a PostgresqlConnectionPool,
    pub postgresql_connection_pool_database_4: &'a PostgresqlConnectionPool,
}
pub type Channel_CheckLinkedNameForExisting = self::channel::check_linked_name_for_existing::Channel_CheckLinkedNameForExisting;
pub type Channel_CheckNameForExisting = self::channel::check_name_for_existing::Channel_CheckNameForExisting;
pub type Channel_Create = self::channel::create::Channel_Create;
pub type Channel_GetManyByNameInSubscriptions = self::channel::get_many_by_name_in_subscriptions::Channel_GetManyByNameInSubscriptions;
pub type Channel_GetManyBySubscription = self::channel::get_many_by_subscription::Channel_GetManyBySubscription;
pub type Channel_GetManyPublicByName = self::channel::get_many_public_by_name::Channel_GetManyPublicByName;
pub type Channel_GetOneById = self::channel::get_one_by_id::Channel_GetOneById;
pub type Channel_Delete = self::channel::delete::Channel_Delete;
pub type Channel_GetManyOwned = self::channel::get_many_owned::Channel_GetManyOwned;
pub type Channel_RefreshChannelToken = self::channel::refresh_channel_token::Channel_RefreshChannelToken;
pub type ChannelPublication1_Create = self::channel_publication1::create::ChannelPublication1_Create;
pub type ChannelPublication1_Delete = self::channel_publication1::delete::ChannelPublication1_Delete;
pub type ChannelPublication1_GetMany = self::channel_publication1::get_many::ChannelPublication1_GetMany;
pub type ChannelPublication1Commentary_Create = self::channel_publication1_commentary::create::ChannelPublication1Commentary_Create;
pub type ChannelPublication1Commentary_Delete = self::channel_publication1_commentary::delete::ChannelPublication1Commentary_Delete;
pub type ChannelPublication1Mark_Create = self::channel_publication1_mark::create::ChannelPublication1Mark_Create;
pub type ChannelPublication1Mark_Delete = self::channel_publication1_mark::delete::ChannelPublication1Mark_Delete;
pub type ChannelPublication1View_Create = self::channel_publication1_view::create::ChannelPublication1View_Create;
pub type ChannelSubscription_Create = self::channel_subscription::create::ChannelSubscription_Create;
pub type ChannelSubscription_Delete = self::channel_subscription::delete::ChannelSubscription_Delete;
pub type User_AuthorizeByFirstStep = self::user::authorize_by_first_step::User_AuthorizeByFirstStep;
pub type User_AuthorizeByLastStep = self::user::authorize_by_last_step::User_AuthorizeByLastStep;
pub type User_CheckEmailForExisting = self::user::check_email_for_existing::User_CheckEmailForExisting;
pub type User_CheckNicknameForExisting = self::user::check_nickname_for_existing::User_CheckNicknameForExisting;
pub type User_DeauthorizeFromAllDevices = self::user::deauthorize_from_all_devices::User_DeauthorizeFromAllDevices;
pub type User_DeauthorizeFromOneDevice = self::user::deauthorize_from_one_device::User_DeauthorizeFromOneDevice;
pub type User_RefreshAccessToken = self::user::refresh_access_token::User_RefreshAccessToken;
pub type User_RegisterByFirstStep = self::user::register_by_first_step::User_RegisterByFirstStep;
pub type User_RegisterByLastStep = self::user::register_by_last_step::User_RegisterByLastStep;
pub type User_RegisterBySecondStep = self::user::register_by_second_step::User_RegisterBySecondStep;
pub type User_ResetPasswordByFirstStep = self::user::reset_password_by_first_step::User_ResetPasswordByFirstStep;
pub type User_ResetPasswordByLastStep = self::user::reset_password_by_last_step::User_ResetPasswordByLastStep;
pub type User_ResetPasswordBySecondStep = self::user::reset_password_by_second_step::User_ResetPasswordBySecondStep;
pub type User_SendEmailForAuthorize = self::user::send_email_for_authorize::User_SendEmailForAuthorize;
pub type User_SendEmailForRegister = self::user::send_email_for_register::User_SendEmailForRegister;
pub type User_SendEmailForResetPassword = self::user::send_email_for_reset_password::User_SendEmailForResetPassword;