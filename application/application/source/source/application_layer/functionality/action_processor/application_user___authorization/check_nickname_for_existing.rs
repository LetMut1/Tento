use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::application_user::{
            ApplicationUser,
            ApplicationUser_Nickname,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            capture::Capture,
        },
        functionality::repository::postgresql::{
            application_user::By1,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::{
    Incoming,
    Outcoming,
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
pub struct ApplicationUser__Authorization___CheckNicknameForExisting;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___CheckNicknameForExisting> {
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
            if !Validator::<ApplicationUser_Nickname>::is_valid(incoming.application_user__nickname.as_str()) {
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
            let is_exist = PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
                &*database_1_postgresql_pooled_connection,
                By1 {
                    application_user__nickname: incoming.application_user__nickname.as_str(),
                },
            )
            .await?;
            let outcoming = Outcoming {
                result: is_exist,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
