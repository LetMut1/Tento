use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use extern_crate::tokio_postgres::Client as Connection;

pub struct ChannelSubscription_PostgresqlRepository;

impl ChannelSubscription_PostgresqlRepository {
    pub async fn is_exist<'a>(
        database_1_connection: &'a Connection,
        channel_id: i64,
        application_user_id: i64
    ) -> Result<bool, ErrorAuditor> {
        return Ok(true); // stub.  -> Пользователь всегда подписан на канал.
    }
}