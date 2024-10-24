use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::user::{
            User,
            User_Email,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::repository::postgresql::{
            user::By2,
            PostgresqlRepository,
        },
    },
};
use forced_crate::action_processor_incoming_outcoming::action_processor::user_authorization::check_email_for_existing::{
    Incoming,
    Outcoming,
};
use crate::infrastructure_layer::data::aggregate_error::{
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
use forced_crate::unified_report::UnifiedReport;
use forced_crate::void::Void;
pub struct UserAuthorization_CheckEmailForExisting;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_CheckEmailForExisting> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Void;
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
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let is_exist = PostgresqlRepository::<User<'_>>::is_exist_2(
                &*database_1_postgresql_pooled_connection,
                By2 {
                    user__email: incoming.user__email.as_str(),
                },
            )
            .await?;
            let outcoming = Outcoming {
                result: is_exist,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
