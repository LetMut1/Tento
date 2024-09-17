use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::Channel_Id,
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
        data::{
            capture::Capture,
        },
        functionality::repository::postgresql::{
            common::By3,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::{
    action_processor::channel___base::get_many_by_subscription::{
        Incoming,
        Outcoming,
        Precedent,
    },
    Common1,
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
pub struct Channel__Base___GetManyBySubscription;
impl ActionProcessor_ for ActionProcessor<Channel__Base___GetManyBySubscription> {
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
        const LIMIT: i16 = 100;
        return async move {
            let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
                inner.environment_configuration,
                &incoming.application_user_access_token_encoded,
            )?
            {
                Extracted::ApplicationUserAccessToken {
                    application_user_access_token: application_user_access_token_,
                } => application_user_access_token_,
                Extracted::ApplicationUserAccessTokenAlreadyExpired => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired));
                }
                Extracted::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList));
                }
            };
            if let Some(requery___channel__id_) = incoming.requery___channel__id {
                if !Validator::<Channel_Id>::is_valid(requery___channel__id_) {
                    return Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            }
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let common_registry = PostgresqlRepository::<Common1>::find_3(
                &*database_1_postgresql_pooled_connection,
                By3 {
                    application_user__id: application_user_access_token.application_user__id,
                    requery___channel__id: incoming.requery___channel__id,
                },
                incoming.limit,
            )
            .await?;
            let outcoming = Outcoming {
                common_registry,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
