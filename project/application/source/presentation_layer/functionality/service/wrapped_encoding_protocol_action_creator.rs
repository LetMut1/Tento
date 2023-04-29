use crate::application_layer::functionality::service::action_processing_delegator::ActionProcessingDelegator;
use crate::application_layer::functionality::service::action_processing_delegator::ConvertibleParts;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
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
use super::action_response_creator::ActionResponseCreator;
use super::request_header_checker::RequestHeaderChecker;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::infrastructure_layer::functionality::service::serializer::Json;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct WrappedEncodingProtocolActionCreator;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl WrappedEncodingProtocolActionCreator {
    pub async fn create_for_json<'a, T, AP, F, API, APO>(
        environment_configuration: &'a EnvironmentConfiguration,
        mut request: Request<Body>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action_processor: AP
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        AP: FnOnce(
            &'a EnvironmentConfiguration,
            Request<Body>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response<Body>>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }

        let bytes = match to_bytes(request.body_mut()).await {
            Ok(bytes_) => bytes_,
            Err(_) => {
                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let incoming = match Serializer::<Json>::deserialize::<API>(bytes.chunk()) {
            Ok(wrapped_incoming_) => wrapped_incoming_,
            Err(_) => {
                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let action_processing_delegator_result = match ActionProcessingDelegator::delegate::<'_, _, _, _, API, APO>(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            ConvertibleParts {
                request,
                action_processor_incoming: incoming
            },
            action_processor
        ).await {
            Ok(action_processor_result_) => action_processor_result_,
            Err(_) => {
                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let unified_report = match action_processing_delegator_result.unified_report {
            Some(unified_report_) => unified_report_,
            None => {
                return ActionResponseCreator::create_from_response_parts(action_processing_delegator_result.response_parts, None);
            }
        };

        let data = match Serializer::<Json>::serialize(&unified_report) {
            Ok(data_) => data_,
            Err(_) => {
                return ActionResponseCreator::create_internal_server_error();
            }
        };

        return ActionResponseCreator::create_from_response_parts(action_processing_delegator_result.response_parts, Some(data));
    }
}