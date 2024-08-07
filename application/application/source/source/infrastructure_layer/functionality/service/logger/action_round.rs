use super::Logger;
use crate::infrastructure_layer::{
    data::{
        aggregate_error::Auditor,
        control_type::{
            ActionRound,
            TokioNonBlockingTask,
        },
        server_workflow_error::{
            Expected,
            Unexpected,
        },
    },
    functionality::service::{
        formatter::Formatter,
        spawner::Spawner,
    },
};
use crate::infrastructure_layer::functionality::service::formatter::action_round::RowData;
impl Logger<ActionRound> {
    pub fn log<'a>(row_data: RowData) -> () {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<ActionRound>::format(&row_data).as_str(),
                );
                return Ok(());
            },
        );
        return ();
    }
    pub fn log_unexpected_auditor<'a>(row_data: RowData, unexpected_auditor: Auditor<Unexpected>) -> () {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<ActionRound>::format_unexpected_auditor(
                        &row_data,
                        &unexpected_auditor,
                    )
                    .as_str(),
                );
                return Ok(());
            },
        );
        return ();
    }
    pub fn log_expected_auditor<'a>(row_data: RowData, expected_auditor: Auditor<Expected>) -> () {
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<ActionRound>::format_expected_auditor(
                        &row_data,
                        &expected_auditor,
                    )
                    .as_str(),
                );
                return Ok(());
            },
        );
        return ();
    }
}