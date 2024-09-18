use super::Logger;
use crate::{
    infrastructure_layer::{
        data::server_workflow_error::{
            Responsive,
            Unresponsive,
        },
        functionality::service::{
            formatter::{
                action_round::RowData,
                Formatter,
            },
            spawner::{
                tokio_non_blocking_task::TokioNonBlockingTask,
                Spawner,
            },
        },
    },
    presentation_layer::functionality::service::processor::action_round::ActionRound,
};
use aggregate_error::Auditor;
impl Logger<ActionRound> {
    pub fn log<'a>(row_data: RowData) -> () {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<ActionRound>::format(&row_data).as_str(),
                );
                return Result::Ok(());
            },
        );
        return ();
    }
    pub fn log_unresponsive_auditor<'a>(row_data: RowData, unresponsive_auditor: Auditor<Unresponsive>) -> () {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<ActionRound>::format_with(
                        &row_data,
                        &unresponsive_auditor,
                    )
                    .as_str(),
                );
                return Result::Ok(());
            },
        );
        return ();
    }
    pub fn log_responsive_auditor<'a>(row_data: RowData, responsive_auditor: Auditor<Responsive>) -> () {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<ActionRound>::format_with(
                        &row_data,
                        &responsive_auditor,
                    )
                    .as_str(),
                );
                return Result::Ok(());
            },
        );
        return ();
    }
}
