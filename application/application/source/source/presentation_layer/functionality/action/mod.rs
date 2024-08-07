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
            }, capture::Capture, control_type::{
                ActionRound, MessagePack, Response
            }, server_workflow_error::ServerWorkflowError
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
use void::Void;
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
#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::data::control_type::Json;
pub struct Action<S> {
    _subject: PhantomData<S>,
}
impl<AP> Action<AP>
where
    ActionProcessor<AP>: ActionProcessor_,
    <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'de> SerdeDeserialize<'de>,
    <ActionProcessor<AP> as ActionProcessor_>::Outcoming: SerdeSerialize,
    <ActionProcessor<AP> as ActionProcessor_>::Precedent: SerdeSerialize,
{
    pub fn run<'a, 'b, 'c, 'd, T>(
        inner: Inner<'b, 'c, 'd>,
        action_processor_inner: &'a ActionProcessorInner<'b, T>,
    ) -> impl Future<Output = Response> + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<ActionRound>::process::<'_, '_, '_, '_, _, AP, MessagePack, MessagePack>(inner, action_processor_inner);
    }
}
#[cfg(feature = "manual_testing")]
impl<AP> Action<AP>
where
    ActionProcessor<AP>: ActionProcessor_,
    <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'de> SerdeDeserialize<'de>,
    <ActionProcessor<AP> as ActionProcessor_>::Outcoming: SerdeSerialize,
    <ActionProcessor<AP> as ActionProcessor_>::Precedent: SerdeSerialize,
{
    pub fn run_<'a, 'b, 'c, 'd, T>(
        inner: Inner<'b, 'c, 'd>,
        action_processor_inner: &'a ActionProcessorInner<'b, T>,
    ) -> impl Future<Output = Response> + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<ActionRound>::process::<'_, '_, '_, '_, _, AP, Json, Json>(inner, action_processor_inner);
    }
}
pub struct Inner<'a, 'b, 'c> {
    pub body: &'a mut Body,
    pub parts: &'a Parts,
    pub route_parameters: &'a Params<'b, 'c>,
}