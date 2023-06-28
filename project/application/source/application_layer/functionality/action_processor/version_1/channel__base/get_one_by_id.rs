use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use crate::domain_layer::data::entity::channel::Channel as EntityChannel;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;
use crate::domain_layer::data::entity::channel::Channel_BackgroundImagePath;
use crate::domain_layer::data::entity::channel::Channel_CoverImagePath;
use crate::domain_layer::data::entity::channel::Channel_Description;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::domain_layer::data::entity::channel::Channel_MarksQuantity;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_Orientation;
use crate::domain_layer::data::entity::channel::Channel_SubscribersQuantity;
use crate::domain_layer::data::entity::channel::Channel_ViewingQuantity;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::domain_layer::data::entity::channel_inner_link::ChannelInnerLink;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink;
use crate::domain_layer::data::entity::channel_subscription::ChannelSubscription;
use crate::domain_layer::functionality::service::application_user_access_token__extractor::ExtractorResult;
use crate::domain_layer::functionality::service::channel__access_modifier_resolver::Channel_AccessModifierResolver;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::functionality::repository::channel_inner_link__postgresql_repository::ChannelInnerLink1;
use crate::infrastructure_layer::functionality::repository::channel_outer_link__postgresql_repository::ChannelOuterLink1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::macro_rules::r#enum;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let extractor_result = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            pushable_environment_configuration,
            &incoming.application_user_access_token_encrypted,
        )
        .await
        {
            Ok(extractor_result_) => extractor_result_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_token = match extractor_result {
            InvalidArgumentResult::Ok {
                subject: extractor_result_,
            } => {
                let application_user_access_token_ = match extractor_result_ {
                    ExtractorResult::ApplicationUserAccessToken {
                        application_user_access_token: application_user_access_token__,
                    } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired),
                            },
                        );
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList),
                            },
                        );
                    }
                };

                application_user_access_token_
            }
            InvalidArgumentResult::InvalidArgument {
                invalid_argument,
            } => {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument,
                    },
                );
            }
        };

        if !Validator::<Channel_Id>::is_valid(incoming.channel_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::Channel_Id,
                },
            );
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel = match PostgresqlRepository::<EntityChannel<'_>>::find_1(
            &*database_1_postgresql_pooled_connection,
            incoming.channel_id,
        )
        .await
        {
            Ok(channel_) => channel_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let channel_ = match channel {
            Some(channel_) => channel_,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::Channel_NotFound),
                    },
                );
            }
        };

        let channel_access_modifier = Channel_AccessModifierResolver::to_representation(channel_.get_access_modifier());

        if let Channel_AccessModifier_::Close = channel_access_modifier {
            let is_exist = match PostgresqlRepository::<ChannelSubscription>::is_exist(
                &*database_1_postgresql_pooled_connection,
                application_user_access_token.get_application_user_id(),
                channel_.get_id(),
            )
            .await
            {
                Ok(is_exist_) => is_exist_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            if !is_exist && application_user_access_token.get_application_user_id().get() != channel_.get_owner().get() {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::Channel_IsClosed),
                    },
                );
            }
        }

        let channel_inner_link_registry = match PostgresqlRepository::<ChannelInnerLink>::find_1(
            &*database_1_postgresql_pooled_connection,
            channel_.get_id(),
            ChannelInnerLink::MAXIMUM_QUANTITY,
        )
        .await
        {
            Ok(channel_inner_link_registry_) => channel_inner_link_registry_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let channel_outer_link_registry = match PostgresqlRepository::<ChannelOuterLink>::find_1(
            &*database_1_postgresql_pooled_connection,
            channel_.get_id(),
            ChannelOuterLink::MAXIMUM_QUANTITY,
        )
        .await
        {
            Ok(channel_outer_link_registry_) => channel_outer_link_registry_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

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
            _channel_created_at,
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
            channel_viewing_quantity,
        };

        let outcoming = Outcoming {
            channel,
            channel_inner_link_registry,
            channel_outer_link_registry,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(outcoming),
            },
        );
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    channel_id: Channel_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    channel: Channel,
    channel_inner_link_registry: Vec<ChannelInnerLink1>,
    channel_outer_link_registry: Vec<ChannelOuterLink1>,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
struct Channel {
    channel_owner: ApplicationUser_Id,
    channel_name: Channel_Name,
    channel_linked_name: Channel_LinkedName,
    channel_description: Option<Channel_Description>,
    channel_access_modifier: Channel_AccessModifier,
    channel_visability_modifier: Channel_VisabilityModifier,
    channel_orientation: Channel_Orientation,
    channel_cover_image_path: Option<Channel_CoverImagePath>,
    channel_background_image_path: Option<Channel_BackgroundImagePath>,
    channel_subscribers_quantity: Channel_SubscribersQuantity,
    channel_marks_quantity: Channel_MarksQuantity,
    channel_viewing_quantity: Channel_ViewingQuantity,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClosed,
    }
);
