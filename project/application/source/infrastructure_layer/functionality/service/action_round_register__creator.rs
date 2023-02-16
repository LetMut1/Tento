use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::action_round_register__postgresql_repository::ActionRoundRegister_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::action_round_register__postgresql_repository::Insert;
use extern_crate::tokio_postgres::Client as Connection;

pub struct ActionRoundRegister_Creator;

impl ActionRoundRegister_Creator {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        action_round_register_route: &'a str,
        action_round_register_status_code: i16,
        action_round_register_context: Option<String>
    ) -> Result<(), ErrorAuditor> {
        let insert = Insert {
            route: action_round_register_route,
            status_code: action_round_register_status_code,
            context: action_round_register_context
        };

        if let Err(mut error) = ActionRoundRegister_PostgresqlRepository::create(database_2_connection, insert).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }
}