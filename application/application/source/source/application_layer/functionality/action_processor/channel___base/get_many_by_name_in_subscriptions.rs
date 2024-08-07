use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::Channel_Name,
        },
        functionality::service::{
            extractor::{
                application_user_access_token::ExtractorResult,
                Extractor,
            },
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
            },
            control_type::Channel__Base___GetManyByNameInSubscriptions,
        },
        functionality::repository::postgresql::{
            common::{
                By2,
                Common1,
            },
            PostgresqlRepository,
        },
    },
};
use crate::infrastructure_layer::data::capture::Capture;
use crate::infrastructure_layer::data::void::Void;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::{
    Incoming,
    Outcoming,
    Precedent,
};
use crate::application_layer::functionality::action_processor::Inner;
use crate::application_layer::functionality::action_processor::ActionProcessor_;
use std::future::Future;
use std::{
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl ActionProcessor_ for ActionProcessor<Channel__Base___GetManyByNameInSubscriptions> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a, T> (
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
                incoming.application_user_access_token_encrypted.as_str(),
            )
            .await?
            {
                ExtractorResult::ApplicationUserAccessToken {
                    application_user_access_token: application_user_access_token_,
                } => application_user_access_token_,
                ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired));
                }
                ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList));
                }
            };
            if incoming.limit <= 0 || incoming.limit > LIMIT {
                return Err(
                    AggregateError::new_invalid_argument_from_outside(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name.as_str()) {
                return Err(
                    AggregateError::new_invalid_argument_from_outside(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if let Some(ref requery___channel__name_) = incoming.requery___channel__name {
                if !Validator::<Channel_Name>::is_valid(requery___channel__name_.as_str()) {
                    return Err(
                        AggregateError::new_invalid_argument_from_outside(
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
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
