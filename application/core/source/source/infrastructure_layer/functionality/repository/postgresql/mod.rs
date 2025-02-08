mod channel;
mod channel_inner_link;
mod channel_outer_link;
mod channel_subscription;
mod common;
mod user;
mod user_access_refresh_token;
mod user_authorization_token;
mod user_device;
mod user_registration_token;
mod user_reset_password_token;
pub use self::{
    channel::{
        By1 as ChannelBy1,
        By2 as ChannelBy2,
        By3 as ChannelBy3,
        Insert1 as ChannelInsert1,
    },
    channel_inner_link::By1 as ChannelInnerLinkBy1,
    channel_outer_link::By1 as ChannelOuterLinkBy1,
    channel_subscription::By1 as ChannelSubscriptionBy1,
    common::{
        By1 as CommonBy1,
        By2 as CommonBy2,
        By3 as CommonBy3,
    },
    user::{
        By1 as UserBy1,
        By2 as UserBy2,
        By3 as UserBy3,
        Insert1 as UserInsert1,
        Update1 as UserUpdate1,
    },
    user_access_refresh_token::{
        By1 as UserAccessRefreshTokenBy1,
        By2 as UserAccessRefreshTokenBy2,
        Update1 as UserAccessRefreshTokenUpdate1,
    },
    user_authorization_token::{
        By1 as UserAuthorizationTokenBy1,
        Update1 as UserAuthorizationTokenUpdate1,
        Update2 as UserAuthorizationTokenUpdate2,
        Update3 as UserAuthorizationTokenUpdate3,
    },
    user_device::Insert1 as UserDeviceInsert1,
    user_registration_token::{
        By1 as UserRegistrationTokenBy1,
        Update1 as UserRegistrationTokenUpdate1,
        Update2 as UserRegistrationTokenUpdate2,
        Update3 as UserRegistrationTokenUpdate3,
        Update5 as UserRegistrationTokenUpdate5,
    },
    user_reset_password_token::{
        By1 as UserResetPasswordTokenBy1,
        Update1 as UserResetPasswordTokenUpdate1,
        Update2 as UserResetPasswordTokenUpdate2,
        Update3 as UserResetPasswordTokenUpdate3,
        Update5 as UserResetPasswordTokenUpdate5,
    },
};
use {
    crate::infrastructure_layer::data::aggregate_error::AggregateError,
    deadpool_postgres::Client,
    std::future::Future,
    std::marker::PhantomData,
    tokio_postgres::types::{
        ToSql,
        Type,
    },
};
pub struct Postgresql<E> {
    _entity: PhantomData<E>,
}
struct ParameterStorage<'a, 'b> {
    parameter_registry: Vec<&'a (dyn ToSql + Sync + 'b)>,
    parameter_type_registry: Vec<Type>,
}
impl<'a, 'b> ParameterStorage<'a, 'b> {
    pub fn new() -> Self {
        return Self {
            parameter_registry: vec![],
            parameter_type_registry: vec![],
        };
    }
    pub fn add<'c>(&'c mut self, parameter_value: &'a (dyn ToSql + Sync + 'b), patameter_type: Type) -> &'c mut Self {
        self.parameter_registry.push(parameter_value);
        self.parameter_type_registry.push(patameter_type);
        return self;
    }
    pub fn get_parameter_registry<'c>(&'c self) -> &'c [&'a (dyn ToSql + Sync + 'b)] {
        return &self.parameter_registry;
    }
    pub fn get_parameter_type_registry<'c>(&'c self) -> &'c [Type] {
        return &self.parameter_type_registry;
    }
}
pub struct Resolver<E> {
    _entity: PhantomData<E>,
}
pub struct Transaction<'a> {
    // Should be &'_ mut for outer requirement.
    client: &'a mut Client,
}
impl<'a> Transaction<'a> {
    pub fn get_client<'b>(&'b self) -> &'b Client {
        return &*self.client;
    }
}
impl Resolver<Transaction<'_>> {
    pub fn start<'a>(
        client: &'a mut Client,
        transaction_isolation_level: IsolationLevel,
    ) -> impl Future<Output = Result<Transaction<'a>, AggregateError>> + Send {
        return async move {
            let mut query = "START TRANSACTION ISOLATION LEVEL".to_string();
            match transaction_isolation_level {
                IsolationLevel::ReadCommitted => {
                    query = format!(
                        "{} READ COMMITTED,READ WRITE,NOT DEFERRABLE;",
                        query.as_str(),
                    );
                }
                IsolationLevel::RepeatableRead => {
                    query = format!(
                        "{} REPEATABLE READ,READ WRITE,NOT DEFERRABLE;",
                        query.as_str(),
                    );
                }
                IsolationLevel::Serializable {
                    read_only,
                    deferrable,
                } => {
                    if read_only && deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ ONLY,DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                    if read_only && !deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ ONLY,NOT DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                    if !read_only && deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ WRITE,DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                    if !read_only && !deferrable {
                        query = format!(
                            "{} SERIALIZABLE,READ WRITE,NOT DEFERRABLE;",
                            query.as_str(),
                        );
                    }
                }
            }
            if let Result::Err(aggregate_error) = crate::result_into_runtime!(
                client.simple_query(query.as_str()).await
            ) {
                let _ = client.simple_query("ROLLBACK;").await;
                return Result::Err(aggregate_error);
            }
            return Result::Ok(
                Transaction {
                    client,
                },
            );
        };
    }
    pub fn commit<'a>(transaction: Transaction<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            if let Result::Err(aggregate_error) = crate::result_into_runtime!(
                transaction.client.simple_query("COMMIT;").await
            ) {
                let _ = transaction.client.simple_query("ROLLBACK;").await;
                return Result::Err(aggregate_error);
            }
            return Result::Ok(());
        };
    }
    pub fn rollback<'a>(transaction: Transaction<'a>) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            crate::result_return_runtime!(transaction.client.simple_query("ROLLBACK;").await);
            return Result::Ok(());
        };
    }
}
pub enum IsolationLevel {
    ReadCommitted,
    RepeatableRead,
    Serializable {
        read_only: bool,
        deferrable: bool,
    },
}
