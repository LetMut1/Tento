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
                    Channel_AccessModifier_,
                    Channel_Id,
                },
                channel_publication1::ChannelPublication1,
                user_access_token::UserAccessToken,
                channel_publication1_token::{
                    ChannelPublication1Token,
                    ChannelPublication1Token_ExpiresAt,
                }
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
                        ChannelBy1,
                        ChannelPublication1By2,
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
        action_processor_incoming_outcoming::action_processor::channel_publication1::get_many::{
            Data,
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct ChannelPublication1_GetMany;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1_GetMany> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            const LIMIT: i16 = 30;
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let (channel__owner, channel__access_modifier) = match Repository::<Postgresql<Channel>>::find_6(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound)),
            };
            if incoming.user_access_token_signed.user__id != channel__owner {
                if Channel_AccessModifier_::Close as i16 == channel__access_modifier {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel__IsClose));
                }
            }
            let rows = Repository::<Postgresql<ChannelPublication1>>::find_1(
                &postgresql_database_3_client,
                ChannelPublication1By2 {
                    channel__id: incoming.channel__id,
                    channel_publication1__created_at: incoming.channel_publication1__created_at,
                    channel_publication1__is_predeleted: false,
                },
                incoming.limit,
            )
            .await?;
            let channel_publication1_token__expires_at = Generator::<ChannelPublication1Token_ExpiresAt>::generate(now)?;
            let mut data_registry: Vec<Data> = Vec::with_capacity(rows.len());
            '_a: for row in rows.iter() {
                let channel_publication1__id = crate::result_return_logic!(row.try_get::<'_, usize, i64>(0));
                data_registry.push(
                    Data {
                        channel_publication1__id,
                        channel_publication1__images_pathes: crate::result_return_logic!(row.try_get::<'_, usize, Vec<String>>(1)),
                        channel_publication1__text: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(2)),
                        channel_publication1__marks_quantity: crate::result_return_logic!(row.try_get::<'_, usize, i64>(3)),
                        channel_publication1__view_quantity: crate::result_return_logic!(row.try_get::<'_, usize, i64>(4)),
                        channel_publication1__created_at: crate::result_return_logic!(row.try_get::<'_, usize, i64>(6)),
                        channel_publication1_mark__created_at: crate::result_return_logic!(row.try_get::<'_, usize, Option<i64>>(7)),
                        channel_publication1_token_signed: Encoder::<ChannelPublication1Token>::encode(
                            &inner.environment_configuration.subject.encryption.private_key,
                            incoming.user_access_token_signed.user__id,
                            channel_publication1__id,
                            crate::result_return_logic!(row.try_get::<'_, usize, i64>(5)),
                            channel_publication1_token__expires_at,
                        )?,
                    },
                );
            }
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        data_registry,
                    },
                ),
            );
        };
    }
}
