pub struct ChannelFeedPublicationMark {
    id: i64,
    application_user_id: i64,
    channel_feed_publication_id: i64,
    r#type: u8
}

impl ChannelFeedPublicationMark {
    pub fn new(
        id: i64,
        application_user_id: i64,
        channel_feed_publication_id: i64,
        r#type: u8
    ) -> Self {
        return Self {
            id,
            application_user_id,
            channel_feed_publication_id,
            r#type
        };
    }

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }

    pub fn get_application_user_id<'a>(&'a self) -> i64 {
        return self.application_user_id;
    }

    pub fn get_channel_feed_publication_id<'a>(&'a self) -> i64 {
        return self.channel_feed_publication_id;
    }

    pub fn get_type<'a>(&'a self) -> u8 {
        return self.r#type;
    }
}