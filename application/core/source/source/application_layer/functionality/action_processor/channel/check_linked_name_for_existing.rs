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
                        ChannelBy2,
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
        action_processor_incoming_outcoming::action_processor::channel::check_linked_name_for_existing::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_CheckLinkedNameForExisting;
impl ActionProcessor_ for ActionProcessor<Channel_CheckLinkedNameForExisting> {
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
            if incoming.user_access_token_signed.user_access_token__expires_at <= Resolver::<UnixTime>::get_now_in_microseconds() {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Validator::<Channel_LinkedName>::is_valid(incoming.channel__linked_name) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let is_exist = Repository::<Postgresql<Channel>>::is_exist_1(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await),
                ChannelBy2 {
                    channel__name: incoming.channel__linked_name,
                },
            )
            .await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        result: is_exist,
                    },
                ),
            );
        };
    }
}
