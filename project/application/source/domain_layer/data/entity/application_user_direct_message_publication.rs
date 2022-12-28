pub struct ApplicationUserDirectMessagePublication {
    id: i64,
    application_user_direct_message_id: i64,
    application_user_id: i64,
    channel_feed_publication_id: i64,
    channel_feed_publication_reaction_id: i64,
    created_at: String
}

impl ApplicationUserDirectMessagePublication {
    pub fn new(
        id: i64,
        application_user_direct_message_id: i64,
        application_user_id: i64,
        channel_feed_publication_id: i64,
        channel_feed_publication_reaction_id: i64,
        created_at: String
    ) -> Self {
        return Self {
            id,
            application_user_direct_message_id,
            application_user_id,
            channel_feed_publication_id,
            channel_feed_publication_reaction_id,
            created_at
        };
    }

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }

    pub fn get_application_user_direct_message_id<'a>(&'a self) -> i64 {
        return self.application_user_direct_message_id;
    }

    pub fn get_application_user_id<'a>(&'a self) -> i64 {
        return self.application_user_id;
    }

    pub fn get_channel_feed_publication_id<'a>(&'a self) -> i64 {
        return self.channel_feed_publication_id;
    }

    pub fn get_channel_feed_publication_reaction_id<'a>(&'a self) -> i64 {
        return self.channel_feed_publication_reaction_id;
    }

    pub fn get_created_at<'a>(&'a self) -> &'a str {
        return self.created_at.as_str();
    }
}