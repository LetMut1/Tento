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
                ResultConverter,
            },
            control_type::Channel__Base___GetManyByNameInSubscriptions,
            environment_configuration::EnvironmentConfiguration,
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
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::{
    Incoming,
    Outcoming,
    Precedent,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
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
impl ActionProcessor<Channel__Base___GetManyByNameInSubscriptions> {
    const LIMIT: i16 = 100;
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming,
    ) -> Result<UnifiedReport<Outcoming, Precedent>, AggregateError>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            environment_configuration,
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
        if incoming.limit <= 0 || incoming.limit > Self::LIMIT {
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
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
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
    }
}
