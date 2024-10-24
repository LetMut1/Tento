use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::user_access_token::UserAccessToken,
        functionality::service::{
            extractor::{
                Extracted,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::repository::postgresql::PostgresqlRepository,
    },
};
use dedicated_crate::action_processor_incoming_outcoming::action_processor::channel::check_name_for_existing::{
    Incoming,
    Outcoming,
    Precedent
};
use crate::domain_layer::data::entity::channel::Channel;
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
use crate::infrastructure_layer::data::aggregate_error::Backtrace;
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::infrastructure_layer::functionality::repository::postgresql::channel::By2;
use dedicated_crate::unified_report::UnifiedReport;
use dedicated_crate::void::Void;
pub struct Channel_CheckNameForExisting;
impl ActionProcessor_ for ActionProcessor<Channel_CheckNameForExisting> {
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
            match Extractor::<UserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    user_access_token: _,
                } => {},
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
            let is_exist = PostgresqlRepository::<Channel<'_>>::is_exist_1(
                &*inner.get_database_1_postgresql_pooled_connection().await?,
                By2 {
                    channel__name: incoming.channel__name.as_str(),
                },
            ).await?;
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