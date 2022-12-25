use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserAccessToken_WorkflowException;
use crate::domain_layer::functionality::service::channel__validator::Channel_Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Channel_PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ApplicationUserAccessToken_Extractor;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
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

pub struct Base;

impl Base {
    const LIMIT: i8 = 30;

    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionHandlerResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_access_token_web_form,
            channel_name,
            requery_channel_name,
            mut limit
        ) = incoming.into_inner();

        match ApplicationUserAccessToken_Extractor::extract(environment_configuration_resolver, application_user_access_token_web_form.as_str()).await {
            Ok(extractor_result) => {
                match extractor_result {
                    ExtractorResult::ApplicationUserAccessToken { application_user_access_token: _ } => {
                        if limit <= 0 || limit > Self::LIMIT {
                            limit = Self::LIMIT;
                        }

                        if !Channel_Validator::is_valid_name(channel_name.as_str()) {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::InvalidArgumentError,
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                        if let Some(ref requery_channel_name_) = requery_channel_name {
                            if !Channel_Validator::is_valid_name(requery_channel_name_.as_str()) {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::InvalidArgumentError,
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }

                        match core_postgresql_connection_pool.get().await {
                            Ok(core_postgresql_pooled_connection) => {
                                match Channel_PostgresqlRepository::per_request_1(
                                    &*core_postgresql_pooled_connection, channel_name.as_str(), &requery_channel_name, limit as i16
                                ).await {
                                    Ok(channel_registry) => {
                                        return Ok(ActionHandlerResult::new_with_outcoming(Outcoming::new(channel_registry)));
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                        return Err(error);
                                    }
                                }
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessToken_WorkflowException::AlreadyExpired));
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessToken_WorkflowException::InApplicationUserAccessTokenBlackList));
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_web_form: String,
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i8
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (String, String, Option<String>, i8) {
        return (
            self.application_user_access_token_web_form,
            self.channel_name,
            self.requery_channel_name,
            self.limit
        );
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    channel_registry: Option<Vec<Channel>>
}

impl Outcoming {
    pub fn new(
        channel_registry: Option<Vec<Channel>>
    ) -> Self {
        return Self {
            channel_registry
        };
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Channel {
    channel_id: i64,
    channel_name: String,
    channel_personalization_image_path: String,
    channel_subscribers_quantity: i64,
    channel_public_marks_quantity: i64,
    channel_hidden_marks_quantity: i64,
    channel_reactions_quantity: i64,
    channel_viewing_quantity: i64,
    channel_created_at: String
}

impl Channel {
    pub fn new(
        channel_id: i64,
        channel_name: String,
        channel_personalization_image_path: String,
        channel_subscribers_quantity: i64,
        channel_public_marks_quantity: i64,
        channel_hidden_marks_quantity: i64,
        channel_reactions_quantity: i64,
        channel_viewing_quantity: i64,
        channel_created_at: String
    ) -> Self {
        return Self {
            channel_id,
            channel_name,
            channel_personalization_image_path,
            channel_subscribers_quantity,
            channel_public_marks_quantity,
            channel_hidden_marks_quantity,
            channel_reactions_quantity,
            channel_viewing_quantity,
            channel_created_at
        };
    }
}