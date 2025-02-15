mod channel;
mod channel_subscription;
mod user;
mod user_access_refresh_token;
mod user_authorization_token;
mod user_device;
mod user_registration_token;
mod user_reset_password_token;
pub use self::{
    By1 as CommonBy1,
    By2 as CommonBy2,
    By3 as CommonBy3,
    channel::{
        By1 as ChannelBy1,
        By2 as ChannelBy2,
        By3 as ChannelBy3,
        Insert1 as ChannelInsert1,
    },
    channel_subscription::By1 as ChannelSubscriptionBy1,
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
    crate::infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::{
            repository::Repository,
            service::counter::{
                Counter,
                Counter_,
            },
        },
    },
    dedicated::action_processor_incoming_outcoming::{
        action_processor::channel::{
            get_many_public_by_name::Data as Data1,
            get_many_by_name_in_subscriptions::Data as Data2,
            get_many_by_subscription::Data as Data3,
        },
        Common1
    },
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
    pub fn new() -> Self {
        return Self {
            parameters: vec![],
            parameters_types: vec![],
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
impl Repository<Postgresql<Common1>> {
    pub fn find_1<'a>(database_1_client: &'a Client, by: By1<'a>, limit: i16) -> impl Future<Output = Result<Vec<Data1>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.access_modifier AS am,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip,\
                    cs.channel__id AS ca \
                FROM \
                    public.channel c \
                LEFT OUTER JOIN \
                    public.channel_subscription cs \
                ON \
                    cs.user__id = $1 \
                    AND c.id = cs.channel__id \
                WHERE \
                    c.visability_modifier = $2 \
                    AND c.name LIKE $3"
                .to_string();
            let mut counter = Counter::<u8>::new(
                3,
                1,
            );
            let wildcard = format!("{}%", by.channel__name,);
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &by.channel__visability_modifier,
                    Type::INT2,
                )
                .add(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by.requery___channel__name {
                query = format!(
                    "{} \
                    AND c.name > ${}",
                    query.as_str(),
                    counter.get_next_value_unchecked(),
                );
                parameter_storage.add(
                    requery___channel__name,
                    Type::TEXT,
                );
            }
            query = format!(
                "{} \
                ORDER BY c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            let mut data_registry: Vec<Data1> = vec![];
            if rows.is_empty() {
                return Result::Ok(data_registry);
            }
            '_a: for row in rows.iter() {
                let common = Data1 {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: by.channel__visability_modifier,
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(4)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    is_user_subscribed: crate::result_return_logic!(row.try_get::<'_, usize, Option<i64>>(6)).is_some(),
                };
                data_registry.push(common);
            }
            return Result::Ok(data_registry);
        };
    }
    pub fn find_2<'a>(database_1_client: &'a Client, by: By2<'a>, limit: i16) -> impl Future<Output = Result<Vec<Data2>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip \
                FROM \
                    public.channel c \
                INNER JOIN \
                    public.channel_subscription cs \
                ON \
                    cs.user__id = $1 \
                    AND c.id = cs.channel__id \
                WHERE c.name LIKE $2"
                .to_string();
            let mut counter = Counter::<u8>::new(
                2,
                1,
            );
            let wildcard = format!("{}%", by.channel__name,);
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage
                .add(
                    &by.user__id,
                    Type::INT8,
                )
                .add(
                    &wildcard,
                    Type::TEXT,
                );
            if let Option::Some(ref requery___channel__name) = by.requery___channel__name {
                query = format!(
                    "{} \
                    AND c.name > ${}",
                    query.as_str(),
                    counter.get_next_value_unchecked(),
                );
                parameter_storage.add(
                    requery___channel__name,
                    Type::TEXT,
                );
            }
            query = format!(
                "{} \
                ORDER BY c.name ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            let mut data_registry: Vec<Data2> = vec![];
            if rows.is_empty() {
                return Result::Ok(data_registry);
            }
            '_a: for row in rows.iter() {
                let data = Data2 {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(4)),
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(6)),
                };
                data_registry.push(data);
            }
            return Result::Ok(data_registry);
        };
    }
    pub fn find_3<'a>(database_1_client: &'a Client, by: By3, limit: i16) -> impl Future<Output = Result<Vec<Data3>, AggregateError>> + Send + use<'a> {
        return async move {
            let mut query = "\
                SELECT \
                    c.id AS i,\
                    c.name AS n,\
                    c.linked_name AS ln,\
                    c.access_modifier AS am,\
                    c.visability_modifier AS vm,\
                    c.cover_image_path AS cip,\
                    c.background_image_path AS bip \
                FROM \
                    public.channel c \
                INNER JOIN \
                    public.channel_subscription cs \
                ON \
                    cs.user__id = $1 \
                    AND c.id = cs.channel__id"
                .to_string();
            let mut counter = Counter::<u8>::new(
                1,
                1,
            );
            let mut parameter_storage = ParameterStorage::new();
            parameter_storage.add(
                &by.user__id,
                Type::INT8,
            );
            let requery___channel__id: i64;
            if let Option::Some(requery___channel__id_) = by.requery___channel__id {
                requery___channel__id = requery___channel__id_;
                query = format!(
                    "{} \
                    WHERE cs.channel__id > ${}",
                    query.as_str(),
                    counter.get_next_value_unchecked(),
                );
                parameter_storage.add(
                    &requery___channel__id,
                    Type::INT8,
                );
            }
            query = format!(
                "{} \
                ORDER BY cs.channel__id ASC \
                LIMIT ${};",
                query.as_str(),
                counter.get_next_value_unchecked(),
            );
            parameter_storage.add(
                &limit,
                Type::INT2,
            );
            let statement = crate::result_return_logic!(
                database_1_client
                .prepare_typed_cached(
                    query.as_str(),
                    parameter_storage.get_parameters_types(),
                )
                .await
            );
            let rows = crate::result_return_runtime!(
                database_1_client
                .query(
                    &statement,
                    parameter_storage.get_parameters(),
                )
                .await
            );
            let mut data_registry: Vec<Data3> = vec![];
            if rows.is_empty() {
                return Result::Ok(data_registry);
            }
            '_a: for row in rows.iter() {
                let data = Data3 {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(4)),
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(6)),
                };
                data_registry.push(data);
            }
            return Result::Ok(data_registry);
        };
    }
}
pub struct By1<'a> {
    pub user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
    pub channel__visability_modifier: i16,
}
pub struct By2<'a> {
    pub user__id: i64,
    pub channel__name: &'a str,
    pub requery___channel__name: Option<&'a str>,
}
pub struct By3 {
    pub user__id: i64,
    pub requery___channel__id: Option<i64>,
}
