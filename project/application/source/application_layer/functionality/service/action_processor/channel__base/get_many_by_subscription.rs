use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::common_postgresql_repository::Common1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
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
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let extractor_result = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            environment_configuration, incoming.application_user_access_token_serialized_form.as_str()
        ).await {
            Ok(extractor_result_) => extractor_result_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token = match extractor_result {
            ArgumentResult::Ok { subject: extractor_result_ } => {
                let application_user_access_token_ = match extractor_result_ {
                    ExtractorResult::ApplicationUserAccessToken { application_user_access_token: application_user_access_token__ } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(
                            ArgumentResult::Ok {
                                subject: ActionProcessorResult::UserWorkflowPrecedent {
                                    user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAccessToken_AlreadyExpired
                                }
                            }
                        );
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(
                            ArgumentResult::Ok {
                                subject: ActionProcessorResult::UserWorkflowPrecedent {
                                    user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList
                                }
                            }
                        );
                    }
                };

                application_user_access_token_
            }
            ArgumentResult::InvalidArgument { invalid_argument } => {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument });
            }
        };

        if let Some(requery_channel_id_) = incoming.requery_channel_id {
            if !Validator::<Channel_Id>::is_valid(requery_channel_id_) {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Id });
            }
        }

        if incoming.limit <= 0 || incoming.limit > Self::LIMIT {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Limit });
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

        let common_registry = match PostgresqlRepository::<Common1>::find_3(
            &*database_1_postgresql_pooled_connection,
            application_user_access_token.get_application_user_id(),
            incoming.requery_channel_id,
            incoming.limit
        ).await {
            Ok(channel_registry_) => channel_registry_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming: Outcoming { common_registry } } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_serialized_form: String,
    requery_channel_id: Option<i64>,
    limit: i16
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    common_registry: Vec<Common1>
}