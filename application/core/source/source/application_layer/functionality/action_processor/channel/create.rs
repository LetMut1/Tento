use {
    crate::{
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
                channel_token::{
                    ChannelToken,
                    ChannelToken_ExpiresAt,
                    ChannelToken_ObfuscationValue,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                validator::Validator,
                generator::Generator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy2,
                        ChannelBy3,
                        ChannelInsert,
                        Postgresql,
                    },
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::create::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_Create;
impl ActionProcessor_ for ActionProcessor<Channel_Create> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<Channel_LinkedName>::is_valid(incoming.channel__linked_name) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            if Repository::<Postgresql<Channel>>::is_exist_1(
                &postgresql_client_database_3,
                ChannelBy2 {
                    channel__name: incoming.channel__name,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NameAlreadyExist));
            }
            if Repository::<Postgresql<Channel>>::is_exist_2(
                &postgresql_client_database_3,
                ChannelBy3 {
                    channel__linked_name: incoming.channel__linked_name,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__LinkedNameAlreadyExist));
            }
            let channel__id = match Repository::<Postgresql<Channel>>::create(
                &postgresql_client_database_3,
                ChannelInsert {
                    channel__owner: incoming.user_access_token_signed.user__id,
                    channel__name: incoming.channel__name,
                    channel__linked_name: incoming.channel__linked_name,
                    channel__description: Option::None,
                    channel__access_modifier: incoming.channel__access_modifier,
                    channel__visability_modifier: incoming.channel__visability_modifier,
                    channel__orientation: vec![].as_slice(),
                    channel__cover_image_path: Option::None,
                    channel__background_image_path: Option::None,
                    channel__subscribers_quantity: 0,
                    channel__created_at: now,
                },
            )
            .await?
            {
                Option::Some(channel__id_) => channel__id_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution)),
            };
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_token_signed: Encoder::<ChannelToken>::encode(
                            &inner.environment_configuration.subject.encryption.private_key,
                            incoming.user_access_token_signed.user__id,
                            channel__id,
                            Generator::<ChannelToken_ObfuscationValue>::generate(),
                            Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                            false,
                            true,
                        )?,
                    },
                ),
            );
        };
    }
}
