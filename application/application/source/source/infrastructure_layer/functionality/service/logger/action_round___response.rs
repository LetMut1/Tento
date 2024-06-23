use super::Logger;
use crate::infrastructure_layer::{
    data::control_type::{
        ActionRound,
        Response,
        TokioNonBlockingTask,
    },
    functionality::service::{
        formatter::Formatter,
        spawner::Spawner,
    },
};
use http::request::Parts;
impl
    Logger<(
        ActionRound,
        Response,
    )>
{
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<(ActionRound, Response)>::format(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                    )
                    .as_str()
                );
                return Ok(());
            },
        );
        return ();
    }
}
