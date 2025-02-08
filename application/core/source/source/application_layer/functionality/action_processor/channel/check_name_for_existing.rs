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
            data::aggregate_error::AggregateError,
            functionality::repository::{
                postgresql::{
                    ChannelBy2,
                    Postgresql,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::check_name_for_existing::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_CheckNameForExisting;
impl ActionProcessor_ for ActionProcessor<Channel_CheckNameForExisting> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            match Extractor::<UserAccessToken<'_>>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    user_access_token: _,
                } => {}
                Extracted::UserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::UserAccessTokenInUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let is_exist = Repository::<Postgresql<Channel<'_>>>::is_exist_1(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                ChannelBy2 {
                    channel__name: incoming.channel__name.as_str(),
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
