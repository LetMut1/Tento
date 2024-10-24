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
        functionality::{repository::postgresql::{
            channel::Insert1,
            PostgresqlRepository,
        }, service::resolver::{UnixTime, Resolver}},
    },
};
use dedicated_crate::action_processor_incoming_outcoming::action_processor::channel::create::{
    Incoming,
    Outcoming,
    Precedent,
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
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::infrastructure_layer::functionality::repository::postgresql::channel::By3;
use crate::infrastructure_layer::functionality::repository::postgresql::channel::By2;
use dedicated_crate::unified_report::UnifiedReport;
use dedicated_crate::void::Void;
pub struct Channel_Create;
impl ActionProcessor_ for ActionProcessor<Channel_Create> {
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
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                inner.environment_configuration,
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
            if !Validator::<Channel_LinkedName>::is_valid(incoming.channel__linked_name.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let database_1_postgresql_connection = &*inner.get_database_1_postgresql_pooled_connection().await?;
            if PostgresqlRepository::<Channel<'_>>::is_exist_1(
                database_1_postgresql_connection,
                By2 {
                    channel__name: incoming.channel__name.as_str(),
                },
            ).await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NameAlreadyExist));
            }
            if PostgresqlRepository::<Channel<'_>>::is_exist_2(
                database_1_postgresql_connection,
                By3 {
                    channel__linked_name: incoming.channel__linked_name.as_str(),
                },
            ).await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_LinkedNameAlreadyExist));
            }
            let channel = PostgresqlRepository::<Channel<'_>>::create_1(
                database_1_postgresql_connection,
                Insert1 {
                    channel__owner: user_access_token.user__id,
                    channel__name: incoming.channel__name,
                    channel__linked_name: incoming.channel__linked_name,
                    channel__description: Option::None,
                    channel__access_modifier: incoming.channel__access_modifier,
                    channel__visability_modifier: incoming.channel__visability_modifier,
                    channel__orientation: vec![],
                    channel__cover_image_path: Option::None,
                    channel__background_image_path: Option::None,
                    channel__subscribers_quantity: 0,
                    channel__marks_quantity: 0,
                    channel__viewing_quantity: 0,
                    channel__created_at: Resolver::<UnixTime>::get_now(),
                },
            ).await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel__id: channel.id,
                    },
                ),
            );
        };
    }
}