use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::channel_inner_link::ChannelInnerLink;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink;
use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription;
use crate::domain_layer::data::entity::channel::Channel as EntityChannel;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::functionality::service::channel__access_modifier_resolver::Channel_AccessModifierResolver;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::channel_inner_link__postgresql_repository::ChannelInnerLink_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::channel_inner_link__postgresql_repository::ChannelInnerLink1;
use crate::infrastructure_layer::functionality::repository::channel_outer_link__postgresql_repository::ChannelOuterLink_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::channel_outer_link__postgresql_repository::ChannelOuterLink1;
use crate::infrastructure_layer::functionality::repository::channel_subscription__postgresql_repository::ChannelSubscription_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
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

        if !Validator::<Channel_Id>::is_valid(incoming.channel_id) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::Channel_Id });
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

        let channel = match PostgresqlRepository::<EntityChannel<'_>>::find_1(
            &*database_1_postgresql_pooled_connection, incoming.channel_id
        ).await {
            Ok(channel_) => channel_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let channel_ = match channel {
            Some(channel_) => channel_,
            None => {
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::Channel_NotFound
                        }
                    }
                );
            }
        };

        let channel_access_modifier = Channel_AccessModifierResolver::to_representation(channel_.get_access_modifier());

        if let Channel_AccessModifier::Close = channel_access_modifier {
            let is_exist = match ChannelSubscription_PostgresqlRepository::<ChannelSubscription>::is_exist(
                &*database_1_postgresql_pooled_connection, application_user_access_token.get_application_user_id(), channel_.get_id(),
            ).await {
                Ok(is_exist_) => is_exist_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            if !is_exist
                && application_user_access_token.get_application_user_id() != channel_.get_owner() {
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::Channel_IsClosed
                        }
                    }
                );
            }
        }

        let channel_inner_link_registry = match ChannelInnerLink_PostgresqlRepository::<ChannelInnerLink>::find_1(
            &*database_1_postgresql_pooled_connection, channel_.get_id(), ChannelInnerLink::MAXIMUM_QUANTITY
        ).await {
            Ok(channel_inner_link_registry_) => channel_inner_link_registry_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let channel_outer_link_registry = match ChannelOuterLink_PostgresqlRepository::<ChannelOuterLink>::find_1(
            &*database_1_postgresql_pooled_connection, channel_.get_id(), ChannelOuterLink::MAXIMUM_QUANTITY
        ).await {
            Ok(channel_outer_link_registry_) => channel_outer_link_registry_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let (
            _channel_id,
            channel_owner,
            channel_name,
            channel_linked_name,
            channel_description,
            channel_access_modifier,
            channel_visability_modifier,
            channel_orientation,
            channel_cover_image_path,
            channel_background_image_path,
            channel_subscribers_quantity,
            channel_marks_quantity,
            channel_viewing_quantity,
            _channel_created_at
        ) = channel_.into_inner();

        let channel = Channel {
            channel_owner,
            channel_name: channel_name.into_owned(),
            channel_linked_name,
            channel_description,
            channel_access_modifier,
            channel_visability_modifier,
            channel_orientation,
            channel_cover_image_path,
            channel_background_image_path,
            channel_subscribers_quantity,
            channel_marks_quantity,
            channel_viewing_quantity
        };

        let outcoming = Outcoming {
            channel,
            channel_inner_link_registry,
            channel_outer_link_registry
        };

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_deserialized_form: String,
    channel_id: i64
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    channel: Channel,
    channel_inner_link_registry: Vec<ChannelInnerLink1>,
    channel_outer_link_registry: Vec<ChannelOuterLink1>,
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
struct Channel {
    channel_owner: i64,
    channel_name: String,
    channel_linked_name: String,
    channel_description: Option<String>,
    channel_access_modifier: i16,
    channel_visability_modifier: i16,
    channel_orientation: Vec<i16>,
    channel_cover_image_path: Option<String>,
    channel_background_image_path: Option<String>,
    channel_subscribers_quantity: i64,
    channel_marks_quantity: i64,
    channel_viewing_quantity: i64
}