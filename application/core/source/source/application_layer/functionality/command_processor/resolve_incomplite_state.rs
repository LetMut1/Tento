use std::time::Duration;
pub use crate::infrastructure_layer::data::environment_configuration::resolve_incomplite_state::ResolveIncompliteState;
use super::{TOKIO_CONFIGURATION_ERROR_MESSAGE_1, TOKIO_CONFIGURATION_ERROR_MESSAGE_2, TOKIO_CONFIGURATION_ERROR_MESSAGE_3, TWO_MIB};
use {
    super::CommandProcessor,
    crate::infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            environment_configuration::{
                EnvironmentConfiguration,
                resolve_incomplite_state::Tokio,
            },
        },
        functionality::service::loader::Loader,
    },
    std::future::Future,
    tokio::runtime::{
        Builder as RuntimeBuilder,
        Runtime,
    },
    tracing::Level,
    tracing_appender::non_blocking::{
        NonBlocking,
        NonBlockingBuilder,
        WorkerGuard,
    },
    tracing_subscriber::FmtSubscriber,
};
#[cfg(feature = "logging_to_file")]
use {
    crate::infrastructure_layer::data::environment_configuration::resolve_incomplite_state::Logging,
    tracing_appender::rolling::{
        RollingFileAppender,
        Rotation,
    },
};
impl CommandProcessor<ResolveIncompliteState> {
    pub fn process<'a>(environment_configuration_file_path: &'a str) -> Result<(), AggregateError> {
        let environment_configuration = Self::initialize_environment(environment_configuration_file_path)?;
        if environment_configuration.subject.system.tokio.worker_threads_quantity == 0
            || environment_configuration.subject.system.tokio.worker_threads_quantity as usize != environment_configuration.subject.system.tokio.affinited_cores.len() {
            crate::new_logic!("The vaule of 'system.tokio.worker_threads_quantity' is zero or is not equal to the quantity of elements in the value of 'system.tokio.affinited_cores'.");
        }
        if environment_configuration.subject.system.tokio.worker_thread_stack_size < (1024 * 1024 * 2) {
            crate::new_logic!("The vaule of 'system.tokio.worker_thread_stack_size' is less than 2MiB.");
        }
        let _worker_guard = {
            #[cfg(feature = "logging_to_file")]
            {
                Self::initialize_logging_to_fileger(&environment_configuration.subject.logging)?
            }
            #[cfg(not(feature = "logging_to_file"))]
            {
                Self::initialize_stdout_logger()
            }
        };
        Self::initialize_tokio_runtime(&environment_configuration.subject.system.tokio)?
            .block_on(Self::resolve_incomplite_state(&environment_configuration))?;
        return Result::Ok(());
    }
    fn initialize_environment<'a>(environment_configuration_file_path: &'a str) -> Result<EnvironmentConfiguration<ResolveIncompliteState>, AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration<ResolveIncompliteState>>::load_from_file(environment_configuration_file_path)?;
        if environment_configuration.subject.system.tokio.worker_threads_quantity == 0 {
            crate::new_logic!(TOKIO_CONFIGURATION_ERROR_MESSAGE_1);
        }
        if environment_configuration.subject.system.tokio.worker_threads_quantity as usize != environment_configuration.subject.system.tokio.affinited_cores.len() {
            crate::new_logic!(TOKIO_CONFIGURATION_ERROR_MESSAGE_2);
        }
        if environment_configuration.subject.system.tokio.worker_thread_stack_size < (TWO_MIB) {
            crate::new_logic!(TOKIO_CONFIGURATION_ERROR_MESSAGE_3);
        }
        return Ok(environment_configuration);
    }
    #[cfg(feature = "logging_to_file")]
    fn initialize_logging_to_fileger<'a>(logging: &'a Logging) -> Result<WorkerGuard, AggregateError> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            logging.directory_path.as_str(),
            logging.file_name_prefix.as_str(),
        );
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(rolling_file_appender);
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Result::Ok(worker_guard);
    }
    #[cfg(not(feature = "logging_to_file"))]
    fn initialize_stdout_logger() -> Result<WorkerGuard, AggregateError> {
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(std::io::stdout());
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Result::Ok(worker_guard);
    }
    fn initialize_tracing_subscriber(non_blocking: NonBlocking) -> Result<(), AggregateError> {
        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_writer(non_blocking)
            .with_file(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();
        crate::result_return_logic!(tracing::subscriber::set_global_default(fmt_subscriber));
        return Result::Ok(());
    }
    fn initialize_tokio_runtime<'a>(tokio: &'a Tokio) -> Result<Runtime, AggregateError> {
        return crate::result_into_runtime!(
            RuntimeBuilder::new_multi_thread()
                .worker_threads(tokio.worker_threads_quantity as usize)
                .max_blocking_threads(1)
                .thread_keep_alive(Duration::from_secs(1))
                .thread_stack_size(tokio.worker_thread_stack_size)
                .enable_all()
                .build()
        );
    }
    fn resolve_incomplite_state<'a>(
        environment_configuration: &'a EnvironmentConfiguration<ResolveIncompliteState>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            // --------------------------------------------------------------------------
            // TODO УДалять с вышедшим сроком экспирации (expires_at) (удалять очень редно, так как нет индекса на поле, по которому будет идти поиск кандидатов.)
            // UserRegistrationToken
            // UserAuthorizationToken
            // UserResetPasswordToken
            // UserAccessRefreshToken
            // ------------------------------------------------------------------------------
            // ------------------------------------------------------------------------------
            // TODO Удалять по can_be_deleted_from и зависимые сущности. Комментарии, Лайки, View и т.п
            // ------------------------------------------------------------------------------
            return Result::Ok(());
        };
    }
}
