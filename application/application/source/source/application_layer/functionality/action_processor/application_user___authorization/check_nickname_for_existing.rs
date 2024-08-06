use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
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
            aggregate_error::{
                AggregateError,
                Backtrace,
            },
            control_type::ApplicationUser__Authorization___CheckNicknameForExisting,
            void::Void,
        },
        functionality::repository::postgresql::{
            application_user::By1,
            PostgresqlRepository,
        },
    },
};
use crate::application_layer::functionality::action_processor::Inner;
use crate::application_layer::functionality::action_processor::ActionProcessor_;
use std::future::Future;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::{
    Incoming,
    Outcoming,
};
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
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___CheckNicknameForExisting> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Void;
    fn process<'a, T> (
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + 'a
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        async move {
            if !Validator::<ApplicationUser_Nickname>::is_valid(incoming.application_user__nickname.as_str()) {
                return Err(
                    AggregateError::new_invalid_argument_from_outside(
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
        }
    }
}
