use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::functionality::service::channel__validator::Channel_Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::common_postgresql_repository::CommonPostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ApplicationUserAccessToken_Extractor;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
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
        let extractor_result = match ApplicationUserAccessToken_Extractor::extract(
            environment_configuration, incoming.application_user_access_token_deserialized_form.as_str()
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

        if let Some(ref search_in_subscription_) = incoming.search_in_subscriptions {
            if search_in_subscription_.limit <= 0 || search_in_subscription_.limit > Self::LIMIT {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Limit });
            }

            if !Channel_Validator::is_valid_name(search_in_subscription_.channel_name.as_str()) {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Name });
            }

            if let Some(ref requery_channel_name_) = search_in_subscription_.requery_channel_name {
                if !Channel_Validator::is_valid_name(requery_channel_name_.as_str()) {
                    return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Name });
                }
            }
        }

        if let Some(ref search_in_all_) = incoming.search_in_all {
            if search_in_all_.limit <= 0 || search_in_all_.limit > Self::LIMIT {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Limit });
            }

            if !Channel_Validator::is_valid_name(search_in_all_.channel_name.as_str()) {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Name });
            }

            if let Some(ref requery_channel_name_) = search_in_all_.requery_channel_name {
                if !Channel_Validator::is_valid_name(requery_channel_name_.as_str()) {
                    return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Name });
                }
            }
        }

        let outcoming = {
            match incoming.search_in_subscriptions {
                Some(search_in_subscription_) => {
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

                    let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

                    let search_in_all_result = match incoming.search_in_all {
                        Some(search_in_all_) => {
                            let channel_registry = match CommonPostgresqlRepository::find_1(
                                database_1_postgresql_connection,
                                search_in_all_.channel_name.as_str(),
                                &search_in_all_.requery_channel_name,
                                search_in_all_.limit
                            ).await {
                                Ok(channel_registry_) => channel_registry_,
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                    return Err(error);
                                }
                            };

                            Some(channel_registry)
                        }
                        None => {
                            None
                        }
                    };

                    let channel_registry = match CommonPostgresqlRepository::find_2(
                        database_1_postgresql_connection,
                        application_user_access_token.get_application_user_id(),
                        search_in_subscription_.channel_name.as_str(),
                        &search_in_subscription_.requery_channel_name,
                        search_in_subscription_.limit
                    ).await {
                        Ok(channel_registry_) => channel_registry_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    Outcoming {
                        search_in_subscriptions_result: Some(channel_registry),
                        search_in_all_result
                    }
                }
                None => {
                    let search_in_all_result = match incoming.search_in_all {
                        Some(search_in_all_) => {
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

                            let channel_registry = match CommonPostgresqlRepository::find_1(
                                &*database_1_postgresql_pooled_connection, search_in_all_.channel_name.as_str(), &search_in_all_.requery_channel_name, search_in_all_.limit
                            ).await {
                                Ok(channel_registry_) => channel_registry_,
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                    return Err(error);
                                }
                            };

                            Some(channel_registry)
                        }
                        None => {
                            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::SearchParameter });
                        }
                    };

                    Outcoming {
                        search_in_all_result,
                        search_in_subscriptions_result: None
                    }
                }
            }
        };

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_deserialized_form: String,
    search_in_subscriptions: Option<SearchInSubscriptions>,
    search_in_all: Option<SearchInAll>
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct SearchInSubscriptions {
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i16
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
struct SearchInAll {
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i16
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    search_in_subscriptions_result: Option<Vec<Channel>>,
    search_in_all_result: Option<Vec<Channel>>
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Channel {
    pub channel_id: i64,
    pub channel_name: String,
    pub channel_linked_name: String,
    pub channel_personalization_image_path: String
}