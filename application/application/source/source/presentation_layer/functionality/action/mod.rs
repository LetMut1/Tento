pub mod health_check;
pub mod route_not_found;
use std::marker::PhantomData;
use crate::{
    application_layer::functionality::action_processor::{ActionProcessor, ActionProcessor_, Inner as ActionProcessorInner},
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            control_type::{
                ActionRound, MessagePack, Response
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
use hyper::{
    body::to_bytes,
    Body,
};
use matchit::Params;
use serde::{
    Serialize as SerdeSerialize,
    Deserialize as SerdeDeserialize,
};
use std::{
    clone::Clone,
    future::Future,
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
use crate::presentation_layer::functionality::service::processor::Processor;
pub struct Action<S> {
    _subject: PhantomData<S>,
}
impl<AP> Action<AP> {
    pub fn run_X <'a, T>(
        inner: Inner<'_, '_, '_>,
        action_processor_inner: &'a ActionProcessorInner<'_, T>,
    ) -> impl Future<Output = Response>
    where
        ActionProcessor<AP>: ActionProcessor_,
        <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'de> SerdeDeserialize<'de>,
        <ActionProcessor<AP> as ActionProcessor_>::Outcoming: SerdeSerialize,
        <ActionProcessor<AP> as ActionProcessor_>::Precedent: SerdeSerialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        // return Processor::<ActionRound>::process::<'a, T, AP, MessagePack, MessagePack>(inner, action_processor_inner);
    }
}
pub struct Inner<'a, 'b, 'c> {
    pub body: &'a mut Body,
    pub parts: &'a Parts,
    pub route_parameters: &'a Params<'b, 'c>,
}