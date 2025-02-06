use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            channel::{
                Channel_Name,
                Channel_VisabilityModifier,
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
            aggregate_error::AggregateError,
            capture::Capture,
        },
        functionality::repository::{
            postgresql::{
                CommonBy1,
                Postgresql,
            },
            Repository,
        },
    },
};
use dedicated::{
    action_processor_incoming_outcoming::{
        action_processor::channel::get_many_public_by_name::{
            Incoming,
            Outcoming,
            Precedent,
        },
        Common1,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct Channel_GetManyPublicByName;
impl ActionProcessor_ for ActionProcessor<Channel_GetManyPublicByName> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        const LIMIT: i16 = 100;
        return async move {
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
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
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if let Option::Some(ref requery___channel__name_) = incoming.requery___channel__name {
                if !Validator::<Channel_Name>::is_valid(requery___channel__name_.as_str()) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            let common_registry = Repository::<Postgresql<Common1>>::find_1(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                CommonBy1 {
                    user__id: user_access_token.user__id,
                    channel__name: incoming.channel__name.as_str(),
                    requery___channel__name: incoming.requery___channel__name.as_deref(),
                    channel__visability_modifier: const { Channel_VisabilityModifier::Public as i16 },
                },
                incoming.limit,
            )
            .await?;
            let outcoming = Outcoming {
                common_registry,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
