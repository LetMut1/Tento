use {
    super::Logger,
    crate::{
        infrastructure_layer::{
            data::{
                aggregate_error::Auditor,
                server_workflow_error::{
                    Responsive,
                    Unresponsive,
                },
            },
            functionality::service::{
                formatter::{
                    Formatter,
                    RowData,
                },
                task_spawner::TaskSpawner,
            },
        },
        presentation_layer::functionality::service::processor::action_round::ActionRound,
    },
};
impl Logger<ActionRound> {
    pub fn log(row_data: RowData) -> () {
        TaskSpawner::spawn_tokio_non_blocking_task_into_background(
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
    pub fn log_unresponsive_auditor(row_data: RowData, unresponsive_auditor: Auditor<Unresponsive>) -> () {
        TaskSpawner::spawn_tokio_non_blocking_task_into_background(
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
    pub fn log_responsive_auditor(row_data: RowData, responsive_auditor: Auditor<Responsive>) -> () {
        TaskSpawner::spawn_tokio_non_blocking_task_into_background(
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
