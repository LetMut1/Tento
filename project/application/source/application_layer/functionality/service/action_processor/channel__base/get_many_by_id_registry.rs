use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::channel__postgresql_repository::Channel_PostgresqlRepository;
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
    const CHANNEL_ID_REGISTRY_LENGTH_LIMIT: usize = 30;

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
        todo!();
        // let extractor_result = match ApplicationUserAccessToken_Extractor::extract(
        //     environment_configuration, incoming.application_user_access_token_deserialized_form.as_str()
        // ).await {
        //     Ok(extractor_result_) => extractor_result_,
        //     Err(mut error) => {
        //         error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

        //         return Err(error);
        //     }
        // };
        // match extractor_result {
        //     ArgumentResult::Ok { subject: extractor_result_ } => {
        //         match extractor_result_ {
        //             ExtractorResult::ApplicationUserAccessToken { application_user_access_token: _ } => {},
        //             ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
        //                 return Ok(
        //                     ArgumentResult::Ok {
        //                         subject: ActionProcessorResult::UserWorkflowPrecedent {
        //                             user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAccessToken_AlreadyExpired
        //                         }
        //                     }
        //                 );
        //             }
        //             ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
        //                 return Ok(
        //                     ArgumentResult::Ok {
        //                         subject: ActionProcessorResult::UserWorkflowPrecedent {
        //                             user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList
        //                         }
        //                     }
        //                 );
        //             }
        //         }
        //     }
        //     ArgumentResult::InvalidArgument { invalid_argument } => {
        //         return Ok(ArgumentResult::InvalidArgument { invalid_argument });
        //     }
        // };

        // // if incoming.channel_id_registry.is_empty() || incoming.channel_id_registry.len() > Self::CHANNEL_ID_REGISTRY_LENGTH_LIMIT {
        // //     return Err(
        // //         ErrorAuditor::new(
        // //             BaseError::InvalidArgumentError,
        // //             BacktracePart::new(line!(), file!(), None)
        // //         )
        // //     );
        // // }

        // let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
        //     Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
        //     Err(error) => {
        //         return Err(
        //             ErrorAuditor::new(
        //                 BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
        //                 BacktracePart::new(line!(), file!(), None)
        //             )
        //         );
        //     }
        // };

        // let channel_registry = match Channel_PostgresqlRepository::per_request_4(
        //     &*database_1_postgresql_pooled_connection, &incoming.channel_id_registry
        // ).await {
        //     Ok(channel_registry_) => channel_registry_,
        //     Err(mut error) => {
        //         error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

        //         return Err(error);
        //     }
        // };

        // return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming: Outcoming { channel_registry } } });
    }
}

#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_deserialized_form: String,
    channel_id_registry: Vec<i64>,
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    channel_registry: Option<Vec<Channel>>
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Channel {
    pub channel_id: i64,
    pub channel_name: String,
    pub channel_personalization_image_path: String,
    pub channel_subscribers_quantity: i64,
    pub channel_public_marks_quantity: i64,
    pub channel_hidden_marks_quantity: i64,
    pub channel_reactions_quantity: i64,
    pub channel_viewing_quantity: i64,
    pub channel_created_at: String
}