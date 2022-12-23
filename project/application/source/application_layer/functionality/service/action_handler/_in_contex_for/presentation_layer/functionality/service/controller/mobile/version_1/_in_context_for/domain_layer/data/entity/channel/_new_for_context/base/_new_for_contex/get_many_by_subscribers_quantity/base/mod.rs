use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Channel_PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ApplicationUserAccessToken_Extractor;
use crate::infrastructure_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::infrastructure_layer::functionality::service::order_convention_resolver::OrderConventionResolver;
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
    const LIMIT_MINIMUM_VALUE: i16 = 300;
    const LIMIT_MAXIMUM_VALUE: i16 = 500;


    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.
    // TODO ЛИБО только от боьшего к меньшему. Либо убирать этот параметр, так как создает много проблем.




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
            channel_subscribers_quantity,       // TODO // TODO // TODO // TODO // TODO  не нужно ли проверять на >=0 ?
            order,
            mut limit
        ) = incoming.into_inner();

        match ApplicationUserAccessToken_Extractor::extract(environment_configuration_resolver, application_user_access_token_web_form.as_str()).await {
            Ok(extractor_result) => {
                match extractor_result {
                    ExtractorResult::ApplicationUserAccessToken { application_user_access_token: _ } => {
                        if !(Self::LIMIT_MINIMUM_VALUE..=Self::LIMIT_MAXIMUM_VALUE).contains(&limit) {
                            limit = Self::LIMIT_MINIMUM_VALUE;
                        }

                        if !OrderConventionResolver::can_convert(order) {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::InvalidArgumentError,
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }

                        match core_postgresql_connection_pool.get().await {
                            Ok(core_postgresql_pooled_connection) => {
                                match Channel_PostgresqlRepository::per_request_3(
                                    &*core_postgresql_pooled_connection, channel_subscribers_quantity, order, limit
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
                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::AlreadyExpired));
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList));
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
    channel_subscribers_quantity: Option<i64>,
    order: i8,
    limit: i16
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (String, Option<i64>, i8, i16) {
        return (
            self.application_user_access_token_web_form,
            self.channel_subscribers_quantity,
            self.order,
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
    channel_subscribers_quantity: i64,
}

impl Channel {
    pub fn new(
        channel_id: i64,
        channel_subscribers_quantity: i64,
    ) -> Self {
        return Self {
            channel_id,
            channel_subscribers_quantity
        };
    }
}