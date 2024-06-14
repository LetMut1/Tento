use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::domain_layer::functionality::service::extractor::application_user_access_token::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::repository::postgresql::common::Common1;
use crate::infrastructure_layer::functionality::repository::postgresql::common::By1;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_public_by_name::Incoming;
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_public_by_name::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_public_by_name::Precedent;
pub use crate::infrastructure_layer::data::control_type::Channel__Base___GetManyPublicByName;

impl ActionProcessor<Channel__Base___GetManyPublicByName> {
    const LIMIT: i16 = 100;

    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Outcoming, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(Backtrace::new(line!(), file!()))?;

        let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(environment_configuration, incoming_.application_user_access_token_encrypted.as_str()).await? {
            Ok(extractor_result) => {
                let application_user_access_token_ = match extractor_result {
                    ExtractorResult::ApplicationUserAccessToken {
                        application_user_access_token: application_user_access_token__,
                    } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired)));
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList)));
                    }
                };

                application_user_access_token_
            }
            Err(invalid_argument_auditor) => {
                return Ok(Err(invalid_argument_auditor));
            }
        };

        if incoming_.limit <= 0 || incoming_.limit > Self::LIMIT {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }

        if !Validator::<Channel_Name>::is_valid(incoming_.channel_name.as_str()) {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }

        if let Some(ref requery_channel_name_) = incoming_.requery_channel_name {
            if !Validator::<Channel_Name>::is_valid(requery_channel_name_.as_str()) {
                return Ok(
                    Err(
                        Auditor::<InvalidArgument>::new(
                            InvalidArgument,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    ),
                );
            }
        }

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        let common_registry = PostgresqlRepository::<Common1>::find_1(
            &*database_1_postgresql_pooled_connection,
            By1 {
                application_user_id: application_user_access_token.application_user_id,
                channel_name: incoming_.channel_name.as_str(),
                requery_channel_name: incoming_.requery_channel_name.as_deref(),
                channel_visability_modifier: Channel_VisabilityModifier::from_representation(Channel_VisabilityModifier::Public),
            },
            incoming_.limit,
        )
        .await?;

        let outcoming = Outcoming {
            common_registry,
        };

        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}
