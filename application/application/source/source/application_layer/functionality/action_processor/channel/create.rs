use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            channel::{
                Channel,
                Channel_LinkedName,
                Channel_Name,
            },
            user_access_token::UserAccessToken,
        },
        functionality::service::{
            extractor::{
                Extracted,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::{
                postgresql::{
                    ChannelBy2,
                    ChannelBy3,
                    ChannelInsert1,
                    Postgresql,
                },
                Repository,
            },
            service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::channel::create::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct Channel_Create;
impl ActionProcessor_ for ActionProcessor<Channel_Create> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                &inner.environment_configuration.encryption.private_key,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    user_access_token: user_access_token_,
                } => user_access_token_,
                Extracted::UserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::UserAccessTokenInUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<Channel_LinkedName>::is_valid(incoming.channel__linked_name.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let postgresql_database_1_client = inner.postgresql_connection_pool_database_1.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            if Repository::<Postgresql<Channel<'_>>>::is_exist_1(
                &postgresql_database_1_client,
                ChannelBy2 {
                    channel__name: incoming.channel__name.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NameAlreadyExist));
            }
            if Repository::<Postgresql<Channel<'_>>>::is_exist_2(
                &postgresql_database_1_client,
                ChannelBy3 {
                    channel__linked_name: incoming.channel__linked_name.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_LinkedNameAlreadyExist));
            }
            let channel = Repository::<Postgresql<Channel<'_>>>::create_1(
                &postgresql_database_1_client,
                ChannelInsert1 {
                    channel__owner: user_access_token.user__id,
                    channel__name: incoming.channel__name,
                    channel__linked_name: incoming.channel__linked_name,
                    channel__description: Option::None,
                    channel__access_modifier: incoming.channel__access_modifier,
                    channel__visability_modifier: incoming.channel__visability_modifier,
                    channel__orientation: vec![],
                    channel__cover_image_path: Option::None,
                    channel__background_image_path: Option::None,
                    channel__subscribers_quantity: 0,
                    channel__marks_quantity: 0,
                    channel__viewing_quantity: 0,
                    channel__created_at: Resolver::<UnixTime>::get_now(),
                },
            )
            .await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel__id: channel.id,
                    },
                ),
            );
        };
    }
}
