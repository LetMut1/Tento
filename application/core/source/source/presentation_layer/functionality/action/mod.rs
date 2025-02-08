mod health_check;
mod route_not_found;
pub use self::route_not_found::RouteNotFound;
#[cfg(feature = "json_for_manual_test")]
use crate::infrastructure_layer::functionality::service::serializer::Json;
use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner as ActionProcessorInner,
        },
        infrastructure_layer::{
            data::control_type::Response,
            functionality::service::serializer::{
                BitCode,
                Deserialize_,
                Serialize_,
            },
        },
        presentation_layer::functionality::service::processor::{
            action_round::ActionRound,
            Processor,
        },
    },
    http::request::Parts,
    hyper::body::Incoming,
    std::{
        future::Future,
        marker::PhantomData,
    },
};
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
    pub fn run<'a, 'b>(
        inner: &'a mut Inner<'b>,
        action_processor_inner: &'a ActionProcessorInner<'b>,
    ) -> impl Future<Output = Response> + Send + use<'a, 'b, AP> {
        return Processor::<ActionRound>::process::<'_, '_, AP, BitCode, BitCode>(
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
    pub fn run_<'a, 'b>(
        inner: &'a mut Inner<'b>,
        action_processor_inner: &'a ActionProcessorInner<'b>,
    ) -> impl Future<Output = Response> + Send + use<'a, 'b, AP> {
        return Processor::<ActionRound>::process::<'_, '_, AP, Json, Json>(
            inner,
            action_processor_inner,
        );
    }
}
pub struct Inner<'a> {
    pub incoming: &'a mut Incoming,
    pub parts: &'a Parts,
}
