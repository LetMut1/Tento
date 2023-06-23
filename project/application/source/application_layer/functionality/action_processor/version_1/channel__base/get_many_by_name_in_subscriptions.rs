use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::common_postgresql_repository::Common1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::macro_rules::r#enum;
use crate::application_layer::data::unified_report::UnifiedReport;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    const LIMIT: i16 = 100;

    pub async fn process<'a, T>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let extractor_result = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            pushable_environment_configuration,
            incoming.application_user_access_token_serialized_form.as_str()
        ).await {
            Ok(extractor_result_) => extractor_result_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token = match extractor_result {
            InvalidArgumentResult::Ok { subject: extractor_result_ } => {
                let application_user_access_token_ = match extractor_result_ {
                    ExtractorResult::ApplicationUserAccessToken { application_user_access_token: application_user_access_token__ } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired)
                            }
                        );
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList)
                            }
                        );
                    }
                };

                application_user_access_token_
            }
            InvalidArgumentResult::InvalidArgument { invalid_argument } => {
                return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument });
            }
        };

        if incoming.limit <= 0 || incoming.limit > Self::LIMIT {
            return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Limit });
        }

        if !Validator::<Channel_Name>::is_valid(&incoming.channel_name) {
            return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Name });
        }

        if let Some(ref requery_channel_name_) = incoming.requery_channel_name {
            if !Validator::<Channel_Name>::is_valid(requery_channel_name_) {
                return Ok(InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Name });
            }
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let common_registry = match PostgresqlRepository::<Common1>::find_2(
            &*database_1_postgresql_pooled_connection,
            application_user_access_token.get_application_user_id(),
            &incoming.channel_name,
            &incoming.requery_channel_name,
            incoming.limit
        ).await {
            Ok(channel_registry_) => channel_registry_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let outcoming = Outcoming { common_registry };

        return Ok(InvalidArgumentResult::Ok { subject: UnifiedReport::filled(outcoming) });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_serialized_form: String,
    channel_name: Channel_Name,
    requery_channel_name: Option<Channel_Name>,
    limit: i16
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    common_registry: Vec<Common1>
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    }
);