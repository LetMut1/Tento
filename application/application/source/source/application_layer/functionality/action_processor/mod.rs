mod user_authorization;
mod channel;
mod channel_subscription;
use crate::infrastructure_layer::data::{
    capture::Capture,
    environment_configuration::EnvironmentConfiguration,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bb8::{
    Pool,
    PooledConnection,
};
use bb8_postgres::PostgresConnectionManager;
use std::{
    future::Future,
    marker::PhantomData,
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub use self::channel_subscription::create::ChannelSubscription_Create;
pub use self::channel::check_linked_name_for_existing::Channel_CheckLinkedNameForExisting;
pub use self::channel::check_name_for_existing::Channel_CheckNameForExisting;
pub use self::channel::create::Channel_Create;
pub use self::channel::get_many_by_name_in_subscriptions::Channel_GetManyByNameInSubscriptions;
pub use self::channel::get_many_by_subscription::Channel_GetManyBySubscription;
pub use self::channel::get_many_public_by_name::Channel_GetManyPublicByName;
pub use self::channel::get_one_by_id::Channel_GetOneById;
pub use self::user_authorization::authorize_by_first_step::UserAuthorization_AuthorizeByFirstStep;
pub use self::user_authorization::authorize_by_last_step::UserAuthorization_AuthorizeByLastStep;
pub use self::user_authorization::check_email_for_existing::UserAuthorization_CheckEmailForExisting;
pub use self::user_authorization::check_nickname_for_existing::UserAuthorization_CheckNicknameForExisting;
pub use self::user_authorization::deauthorize_from_all_devices::UserAuthorization_DeauthorizeFromAllDevices;
pub use self::user_authorization::deauthorize_from_one_device::UserAuthorization_DeauthorizeFromOneDevice;
pub use self::user_authorization::refresh_access_token::UserAuthorization_RefreshAccessToken;
pub use self::user_authorization::register_by_first_step::UserAuthorization_RegisterByFirstStep;
pub use self::user_authorization::register_by_last_step::UserAuthorization_RegisterByLastStep;
pub use self::user_authorization::register_by_second_step::UserAuthorization_RegisterBySecondStep;
pub use self::user_authorization::reset_password_by_first_step::UserAuthorization_ResetPasswordByFirstStep;
pub use self::user_authorization::reset_password_by_last_step::UserAuthorization_ResetPasswordByLastStep;
pub use self::user_authorization::reset_password_by_second_step::UserAuthorization_ResetPasswordBySecondStep;
pub use self::user_authorization::send_email_for_authorize::UserAuthorization_SendEmailForAuthorize;
pub use self::user_authorization::send_email_for_register::UserAuthorization_SendEmailForRegister;
pub use self::user_authorization::send_email_for_reset_password::UserAuthorization_SendEmailForResetPassword;
pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}
pub trait ActionProcessor_ {
    type Incoming;
    type Outcoming;
    type Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send;
}
pub struct Inner<'a, T>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub environment_configuration: &'static EnvironmentConfiguration,
    pub database_1_postgresql_connection_pool: &'a Pool<PostgresConnectionManager<T>>,
    pub database_2_postgresql_connection_pool: &'a Pool<PostgresConnectionManager<T>>,
}
impl<'a, T> Inner<'a, T>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub fn get_database_1_postgresql_pooled_connection<'b>(&'b self) -> impl Future<Output = Result<PooledConnection<PostgresConnectionManager<T>>, AggregateError>> + Send {
        return async move {
            return self.database_1_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        };
    }
    pub fn get_database_2_postgresql_pooled_connection<'b>(&'b self) -> impl Future<Output = Result<PooledConnection<PostgresConnectionManager<T>>, AggregateError>> + Send {
        return async move {
            return self.database_2_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        };
    }
}
