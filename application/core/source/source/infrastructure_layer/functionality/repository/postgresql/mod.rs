mod channel;
mod channel_publication1;
mod channel_publication1_commentary;
mod channel_publication1_mark;
mod channel_publication1_view;
mod channel_subscription;
mod user;
mod user_access_refresh_token;
mod user_authorization_token;
mod user_device;
mod user_registration_token;
mod user_reset_password_token;
mod channel_publication1_delayed_deletion;
pub use self::{
    channel::{
        By1 as ChannelBy1,
        By2 as ChannelBy2,
        By3 as ChannelBy3,
        By4 as ChannelBy4,
        By5 as ChannelBy5,
        By6 as ChannelBy6,
        Insert as ChannelInsert,
    },
    channel_publication1::{
        By1 as ChannelPublication1By1,
        By2 as ChannelPublication1By2,
        Insert as ChannelPublication1Insert,
    },
    channel_publication1_delayed_deletion::Insert as ChannelPublication1DelayedDeletionInsert,
    channel_publication1_commentary::{
        By as ChannelPublication1CommentaryBy,
        Insert as ChannelPublication1CommentaryInsert,
        Update as ChannelPublication1CommentaryUpdate,
    },
    channel_publication1_mark::{
        By as ChannelPublication1MarkBy,
        Insert as ChannelPublication1MarkInsert,
    },
    channel_publication1_view::Insert as ChannelPublication1ViewInsert,
    channel_subscription::{
        By as ChannelSubscriptionBy,
        Insert as ChannelSubscriptionInsert,
    },
    user::{
        By1 as UserBy1,
        By2 as UserBy2,
        By3 as UserBy3,
        Insert1 as UserInsert1,
        Insert2 as UserInsert2,
        Update as UserUpdate,
    },
    user_access_refresh_token::{
        By1 as UserAccessRefreshTokenBy1,
        By2 as UserAccessRefreshTokenBy2,
        Insert as UserAccessRefreshTokenInsert,
        Update as UserAccessRefreshTokenUpdate,
    },
    user_authorization_token::{
        By as UserAuthorizationTokenBy,
        Insert as UserAuthorizationTokenInsert,
        Update1 as UserAuthorizationTokenUpdate1,
        Update2 as UserAuthorizationTokenUpdate2,
        Update3 as UserAuthorizationTokenUpdate3,
    },
    user_device::Insert as UserDeviceInsert,
    user_registration_token::{
        By as UserRegistrationTokenBy,
        Insert as UserRegistrationTokenInsert,
        Update1 as UserRegistrationTokenUpdate1,
        Update2 as UserRegistrationTokenUpdate2,
        Update3 as UserRegistrationTokenUpdate3,
        Update5 as UserRegistrationTokenUpdate5,
    },
    user_reset_password_token::{
        By as UserResetPasswordTokenBy,
        Insert as UserResetPasswordTokenInsert,
        Update1 as UserResetPasswordTokenUpdate1,
        Update2 as UserResetPasswordTokenUpdate2,
        Update3 as UserResetPasswordTokenUpdate3,
        Update5 as UserResetPasswordTokenUpdate5,
    },
};
use {
    crate::infrastructure_layer::data::aggregate_error::AggregateError,
    deadpool_postgres::Client,
    std::{
        future::Future,
        marker::PhantomData,
    },
    tokio_postgres::types::{
        ToSql,
        Type,
    },
};
pub struct Postgresql<E> {
    _entity: PhantomData<E>,
}
struct ParameterStorage<'a, 'b> {
    parameters: Vec<&'a (dyn ToSql + Sync + 'b)>,
    parameters_types: Vec<Type>,
}
impl<'a, 'b> ParameterStorage<'a, 'b> {
    pub fn new(capacity: usize) -> Self {
        return Self {
            parameters: Vec::with_capacity(capacity),
            parameters_types: Vec::with_capacity(capacity),
        };
    }
    pub fn add<'c>(&'c mut self, parameter_value: &'a (dyn ToSql + Sync + 'b), patameter_type: Type) -> &'c mut Self {
        self.parameters.push(parameter_value);
        self.parameters_types.push(patameter_type);
        return self;
    }
    pub fn get_parameters<'c>(&'c self) -> &'c [&'a (dyn ToSql + Sync + 'b)] {
        return &self.parameters;
    }
    pub fn get_parameters_types<'c>(&'c self) -> &'c [Type] {
        return &self.parameters_types;
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
    pub fn start<'a>(client: &'a mut Client, transaction_isolation_level: IsolationLevel) -> impl Future<Output = Result<Transaction<'a>, AggregateError>> + Send {
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
