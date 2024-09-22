use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::{
                Channel as EntityChannel,
                Channel_AccessModifier,
                Channel_Id,
            },
            channel_inner_link::ChannelInnerLink,
            channel_outer_link::ChannelOuterLink,
            channel_subscription::ChannelSubscription,
        },
        functionality::service::{
            extractor::{
                application_user_access_token::Extracted,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::repository::postgresql::{
            channel::By1 as By1___,
            channel_inner_link::By1 as By1__,
            channel_outer_link::By1 as By1_,
            channel_subscription::By1,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::{
    action_processor::channel___base::create::{
        Incoming,
        Outcoming,
        Precedent,
    },
    Channel2,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct Channel__Base___Create;
impl ActionProcessor_ for ActionProcessor<Channel__Base___Create> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )? {
                Extracted::ApplicationUserAccessToken {
                    application_user_access_token: application_user_access_token_,
                } => application_user_access_token_,
                Extracted::ApplicationUserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired));
                }
                Extracted::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList));
                }
            };











            todo!();





            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
