use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::Channel_Name,
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
            common::By2,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::{
    action_processor::channel___base::get_many_by_name_in_subscriptions::{
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
pub struct Channel__Base___GetManyByNameInSubscriptions;
impl ActionProcessor_ for ActionProcessor<Channel__Base___GetManyByNameInSubscriptions> {
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
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
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
            if let Some(ref requery___channel__name_) = incoming.requery___channel__name {
                if !Validator::<Channel_Name>::is_valid(requery___channel__name_.as_str()) {
                    return Result::Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let common_registry = PostgresqlRepository::<Common1>::find_2(
                &*database_1_postgresql_pooled_connection,
                By2 {
                    application_user__id: application_user_access_token.application_user__id,
                    channel__name: incoming.channel__name.as_str(),
                    requery___channel__name: incoming.requery___channel__name.as_deref(),
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
