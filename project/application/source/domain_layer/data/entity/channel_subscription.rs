pub struct ChannelSubscription {
    application_user_id: i64,
    channel_id: i64,
    created_at: String
}

impl ChannelSubscription {
    pub fn new(
        application_user_id: i64,
        channel_id: i64,
        created_at: String
    ) -> Self {
        return Self {
            application_user_id,
            channel_id,
            created_at
        };
    }
}