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
                    Channel_Id,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy6,
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
        action_processor_incoming_outcoming::action_processor::channel::get_many_by_subscription::{
            Data,
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetManyBySubscription;
impl ActionProcessor_ for ActionProcessor<Channel_GetManyBySubscription> {
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
            if incoming.user_access_token_signed.user_access_token__expires_at <= Resolver::<UnixTime>::get_now_in_seconds() {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
            }
            if let Option::Some(requery___channel__id_) = incoming.requery___channel__id {
                if !Validator::<Channel_Id>::is_valid(requery___channel__id_) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            const LIMIT: i16 = 100;
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(crate::new_invalid_argument!());
            }
            let rows = Repository::<Postgresql<Channel>>::find_5(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelBy6 {
                    user__id: incoming.user_access_token_signed.user__id,
                    requery___channel__id: incoming.requery___channel__id,
                },
                incoming.limit,
            )
            .await?;
            let mut data_registry: Vec<Data> = Vec::with_capacity(rows.len());
            '_a: for row in rows.iter() {
                let data = Data {
                    channel__id: crate::result_return_logic!(row.try_get::<'_, usize, i64>(0)),
                    channel__name: crate::result_return_logic!(row.try_get::<'_, usize, String>(1)),
                    channel__linked_name: crate::result_return_logic!(row.try_get::<'_, usize, String>(2)),
                    channel__access_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(3)),
                    channel__visability_modifier: crate::result_return_logic!(row.try_get::<'_, usize, i16>(4)),
                    channel__cover_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(5)),
                    channel__background_image_path: crate::result_return_logic!(row.try_get::<'_, usize, Option<String>>(6)),
                };
                data_registry.push(data);
            }
            let outcoming = Outcoming {
                data_registry,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
