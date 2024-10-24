mod health_check;
mod route_not_found;
#[cfg(feature = "json_for_manual_test")]
use crate::infrastructure_layer::functionality::service::serializer::Json;
use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner as ActionProcessorInner,
    },
    infrastructure_layer::{
        data::{
            capture::Capture,
            control_type::Response,
        },
        functionality::service::serializer::BitCode,
    },
    presentation_layer::functionality::service::processor::{
        action_round::ActionRound,
        Processor,
    },
};
use crate::infrastructure_layer::functionality::service::serializer::Serialize_;
use crate::infrastructure_layer::functionality::service::serializer::Deserialize_;
use http::request::Parts;
pub use self::route_not_found::RouteNotFound;
use hyper::body::Incoming;
use std::{
    future::Future,
    marker::PhantomData,
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use dedicated_crate::void::Void;
pub struct Action<S> {
    _subject: PhantomData<S>,
}
impl<AP> Action<AP>
where
    ActionProcessor<AP>: ActionProcessor_,
    <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'a> Deserialize_<'a>,
    <ActionProcessor<AP> as ActionProcessor_>::Outcoming: Serialize_,
    <ActionProcessor<AP> as ActionProcessor_>::Precedent: Serialize_,
{
    pub fn run<'a, 'b, T>(
        inner: &'a mut Inner<'b>,
        action_processor_inner: &'a ActionProcessorInner<'b, T>,
    ) -> impl Future<Output = Response>
           + Send
           + Capture<(
        &'a Void,
        &'b Void,
    )>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<ActionRound>::process::<'_, '_, _, AP, BitCode, BitCode>(
            inner,
            action_processor_inner,
        );
    }
}
#[cfg(feature = "action_for_manual_test")]
impl<AP> Action<AP>
where
    ActionProcessor<AP>: ActionProcessor_,
    <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'a> Deserialize_<'a>,
    <ActionProcessor<AP> as ActionProcessor_>::Outcoming: Serialize_,
    <ActionProcessor<AP> as ActionProcessor_>::Precedent: Serialize_,
{
    pub fn run_<'a, 'b, T>(
        inner: &'a mut Inner<'b>,
        action_processor_inner: &'a ActionProcessorInner<'b, T>,
    ) -> impl Future<Output = Response>
           + Send
           + Capture<(
        &'a Void,
        &'b Void,
    )>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<ActionRound>::process::<'_, '_, _, AP, Json, Json>(
            inner,
            action_processor_inner,
        );
    }
}
pub struct Inner<'a> {
    pub incoming: &'a mut Incoming,
    pub parts: &'a Parts,
}
