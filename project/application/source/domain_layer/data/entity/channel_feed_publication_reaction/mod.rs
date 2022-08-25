pub struct ChannelFeedPublicationReaction {
    id: i64,
    channel_feed_publication_id: i64,
    application_user_id: i64,
    content_type: u8,
    content_type_component: String,
    public_marks_quantity: i64,
    created_at: String
}

impl ChannelFeedPublicationReaction {
    pub fn new(
        id: i64,
        channel_feed_publication_id: i64,
        application_user_id: i64,
        content_type: u8,
        content_type_component: String,
        public_marks_quantity: i64,
        created_at: String
    ) -> Self {
        return Self {
            id,
            channel_feed_publication_id,
            application_user_id,
            content_type,
            content_type_component,
            public_marks_quantity,
            created_at
        };
    }

    pub fn get_id<'a>(
        &'a self
    ) -> i64 {
        return self.id;
    }

    pub fn get_channel_feed_publication_id<'a>(
        &'a self
    ) -> i64 {
        return self.channel_feed_publication_id;
    }

    pub fn get_application_user_id<'a>(
        &'a self
    ) -> i64 {
        return self.application_user_id;
    }

    pub fn get_content_type<'a>(
        &'a self
    ) -> u8 {
        return self.content_type;
    }

    pub fn get_content_type_component<'a>(
        &'a self
    ) -> &'a str {
        return self.content_type_component.as_str();
    }

    pub fn get_public_marks_quantity<'a>(
        &'a self
    ) -> i64 {
        return self.public_marks_quantity;
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }
}