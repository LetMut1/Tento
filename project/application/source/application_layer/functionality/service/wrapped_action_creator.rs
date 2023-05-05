use crate::application_layer::functionality::service::action_delegator::ActionDelegator;
use crate::application_layer::functionality::service::action_delegator::ConvertibleParts;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::Response;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::body::to_bytes;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize as SerdeSerialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::ops::FnOnce;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct WrappedActionCreator;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl WrappedActionCreator {
    pub async fn create_for_json<'a, SF, WSF, T, A, F, API, APO>(
        environment_configuration: &'a EnvironmentConfiguration,
        mut request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action: A
    ) -> Response
    where
        Serializer<SF>: Serialize,
        Serializer<WSF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        A: FnOnce(
            &'a EnvironmentConfiguration,
            Request,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>
    {
        if !Validator::<Request>::is_valid(&request) {
            return Creator::<Response>::create_bad_request();
        }

        let bytes = match to_bytes(request.body_mut()).await {
            Ok(bytes_) => bytes_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let incoming = match Serializer::<SF>::deserialize::<API>(bytes.chunk()) {
            Ok(wrapped_incoming_) => wrapped_incoming_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let action_processing_delegator_result = match ActionDelegator::delegate::<'_, WSF, _, _, _, API, APO>(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            ConvertibleParts {
                request,
                action_processor_incoming: incoming
            },
            action
        ).await {
            Ok(action_processor_result_) => action_processor_result_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let unified_report = match action_processing_delegator_result.unified_report {
            Some(unified_report_) => unified_report_,
            None => {
                return Creator::<Response>::create_from_response_parts(action_processing_delegator_result.response_parts, None);
            }
        };

        let data = match Serializer::<SF>::serialize(&unified_report) {
            Ok(data_) => data_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        return Creator::<Response>::create_from_response_parts(action_processing_delegator_result.response_parts, Some(data));
    }
}