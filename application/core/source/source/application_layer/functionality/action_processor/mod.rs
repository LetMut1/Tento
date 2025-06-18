mod channel;
mod channel_publication1;
mod channel_publication1_commentary;
mod channel_publication1_commentary_mark;
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
pub type Channel_CheckLinkedNameForExisting = self::channel::check_linked_name_for_existing::CheckLinkedNameForExisting;
pub type Channel_CheckNameForExisting = self::channel::check_name_for_existing::CheckNameForExisting;
pub type Channel_Create = self::channel::create::Create;
pub type Channel_GetManyByNameInSubscriptions = self::channel::get_many_by_name_in_subscriptions::GetManyByNameInSubscriptions;
pub type Channel_GetManyBySubscription = self::channel::get_many_by_subscription::GetManyBySubscription;
pub type Channel_GetManyPublicByName = self::channel::get_many_public_by_name::GetManyPublicByName;
pub type Channel_GetOneById = self::channel::get_one_by_id::GetOneById;
pub type Channel_Delete = self::channel::delete::Delete;
pub type Channel_GetManyOwned = self::channel::get_many_owned::GetManyOwned;
pub type Channel_RefreshChannelToken = self::channel::refresh_channel_token::RefreshChannelToken;
pub type ChannelPublication1_Create = self::channel_publication1::create::Create;
pub type ChannelPublication1_Delete = self::channel_publication1::delete::Delete;
pub type ChannelPublication1_GetMany = self::channel_publication1::get_many::GetMany;
pub type ChannelPublication1Commentary_Create = self::channel_publication1_commentary::create::Create;
pub type ChannelPublication1Commentary_Delete = self::channel_publication1_commentary::delete::Delete;
pub type ChannelPublication1Mark_Create = self::channel_publication1_mark::create::Create;
pub type ChannelPublication1Mark_Delete = self::channel_publication1_mark::delete::Delete;
pub type ChannelPublication1View_Create = self::channel_publication1_view::create::Create;
pub type ChannelSubscription_Create = self::channel_subscription::create::Create;
pub type ChannelSubscription_Delete = self::channel_subscription::delete::Delete;
pub type User_AuthorizeByFirstStep = self::user::authorize_by_first_step::AuthorizeByFirstStep;
pub type User_AuthorizeByLastStep = self::user::authorize_by_last_step::AuthorizeByLastStep;
pub type User_CheckEmailForExisting = self::user::check_email_for_existing::CheckEmailForExisting;
pub type User_CheckNicknameForExisting = self::user::check_nickname_for_existing::CheckNicknameForExisting;
pub type User_DeauthorizeFromAllDevices = self::user::deauthorize_from_all_devices::DeauthorizeFromAllDevices;
pub type User_DeauthorizeFromOneDevice = self::user::deauthorize_from_one_device::DeauthorizeFromOneDevice;
pub type User_RefreshAccessToken = self::user::refresh_access_token::RefreshAccessToken;
pub type User_RegisterByFirstStep = self::user::register_by_first_step::RegisterByFirstStep;
pub type User_RegisterByLastStep = self::user::register_by_last_step::RegisterByLastStep;
pub type User_RegisterBySecondStep = self::user::register_by_second_step::RegisterBySecondStep;
pub type User_ResetPasswordByFirstStep = self::user::reset_password_by_first_step::ResetPasswordByFirstStep;
pub type User_ResetPasswordByLastStep = self::user::reset_password_by_last_step::ResetPasswordByLastStep;
pub type User_ResetPasswordBySecondStep = self::user::reset_password_by_second_step::ResetPasswordBySecondStep;
pub type User_SendEmailForAuthorize = self::user::send_email_for_authorize::SendEmailForAuthorize;
pub type User_SendEmailForRegister = self::user::send_email_for_register::SendEmailForRegister;
pub type User_SendEmailForResetPassword = self::user::send_email_for_reset_password::SendEmailForResetPassword;
