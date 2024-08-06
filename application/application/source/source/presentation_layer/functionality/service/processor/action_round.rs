use std::marker::PhantomData;
use crate::{
    application_layer::{
        functionality::action_processor::{ActionProcessor, ActionProcessor_, Inner as ActionProcessorInner},
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            control_type::{
                ActionRound,
                Response,
            },
            server_workflow_error::ServerWorkflowError,
        },
        functionality::service::{
            creator::Creator,
            logger::Logger,
            serializer::{
                Serialize,
                Serializer,
            },
            validator::Validator,
        },
    },
};
use bytes::Buf;
use http::request::Parts;
use serde::{
    Serialize as SerdeSerialize,
    Deserialize as SerdeDeserialize,
};
use crate::presentation_layer::functionality::action::Inner;

use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use super::Processor;
impl Processor<ActionRound> {
    pub async fn process<'a, T, AP, SS, SD>(
        inner: Inner<'_, '_, '_>,
        action_processor_inner: &'a ActionProcessorInner<'_, T>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        ActionProcessor<AP>: ActionProcessor_,
        <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'de> SerdeDeserialize<'de>,
        <ActionProcessor<AP> as ActionProcessor_>::Outcoming: SerdeSerialize,
        <ActionProcessor<AP> as ActionProcessor_>::Precedent: SerdeSerialize,
        Serializer<SS>: Serialize,
        Serializer<SD>: Serialize,
        'a: 'a,
    {
        let future = async {
            if !Validator::<Parts>::is_valid(inner.parts) {
                return Result::<Vec<u8>, AggregateError>::Err(
                    AggregateError::new_invalid_argument_from_outside(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let bytes = hyper::body::to_bytes(inner.body).await.into_invalid_argument_from_outside_and_client_code(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let incoming = Serializer::<SS>::deserialize::<'_, <ActionProcessor<AP> as ActionProcessor_>::Incoming>(bytes.chunk())?;
            let unified_report = ActionProcessor::<AP>::process(action_processor_inner, incoming).await?;
            return Serializer::<SD>::serialize(&unified_report);
        };
        return match future.await {
            Ok(data) => {
                let response = Creator::<Response>::create_ok(data);
                Logger::<ActionRound>::log(
                    inner.parts,
                    &response,
                );
                response
            }
            Err(aggregate_error) => {
                let response = match ServerWorkflowError::new(aggregate_error) {
                    ServerWorkflowError::Unexpected {
                        unexpected_auditor,
                    } => {
                        let response_ = Creator::<Response>::create_internal_server_error();
                        Logger::<ActionRound>::log_unexpected_auditor(
                            inner.parts,
                            &response_,
                            unexpected_auditor,
                        );
                        response_
                    }
                    ServerWorkflowError::Expected {
                        expected_auditor,
                    } => {
                        let response_ = Creator::<Response>::create_bad_request();
                        Logger::<ActionRound>::log_expected_auditor(
                            inner.parts,
                            &response_,
                            expected_auditor,
                        );
                        response_
                    }
                };
                response
            }
        };
    }
}
