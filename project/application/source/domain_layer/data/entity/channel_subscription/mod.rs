pub struct ChannelSubscription {
    id: i64,
    channel_id: i64,
    application_user_id: i64,
    created_at: String
}

impl ChannelSubscription {
    pub fn new(
        id: i64,
        channel_id: i64,
        application_user_id: i64,
        created_at: String
    ) -> Self {
        return Self {
            id,
            channel_id,
            application_user_id,
            created_at
        };
    }

    pub fn get_id<'a>(
        &'a self
    ) -> i64 {
        return self.id;
    }

    pub fn get_application_user_id<'a>(
        &'a self
    ) -> i64 {
        return self.application_user_id;
    }

    pub fn get_channel_id<'a>(
        &'a self
    ) -> i64 {
        return self.channel_id;
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }
}