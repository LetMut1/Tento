use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user_access_token::ApplicationUserAccessToken,
            channel::Channel_Id,
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
            alternative_workflow::{
                AlternativeWorkflow,
                OptionConverter,
                ResultConverter,
            },
            auditor::Backtrace,
            control_type::Channel__Base___GetManyBySubscription,
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::repository::postgresql::{
            common::{
                By3,
                Common1,
            },
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_subscription::{
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
impl ActionProcessor<Channel__Base___GetManyBySubscription> {
    const LIMIT: i16 = 100;
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Outcoming, Precedent>, AlternativeWorkflow>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.into_internal_logic_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(
            environment_configuration,
            incoming_.application_user_access_token_encrypted.as_str(),
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
        if let Some(requery___channel__id_) = incoming_.requery___channel__id {
            if !Validator::<Channel_Id>::is_valid(requery___channel__id_) {
                return Err(
                    AlternativeWorkflow::new_external_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        }
        if incoming_.limit <= 0 || incoming_.limit > Self::LIMIT {
            return Err(
                AlternativeWorkflow::new_external_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let common_registry = PostgresqlRepository::<Common1>::find_3(
            &*database_1_postgresql_pooled_connection,
            By3 {
                application_user__id: application_user_access_token.application_user__id,
                requery___channel__id: incoming_.requery___channel__id,
            },
            incoming_.limit,
        )
        .await?;
        let outcoming = Outcoming {
            common_registry,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
