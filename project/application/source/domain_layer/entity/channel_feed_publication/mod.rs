pub struct ChannelFeedPublication {
    id: Option<i64>,
    channel_id: i64,
    application_user_channel_administrator_id: i64,
    content_type: u8,
    content_type_component: String,
    content_type_component_preview: String,
    public_marks_quantity: i64,
    hidden_marks_quantity: i64,
    reactions_quantity: i64,
    viewing_quantity: i64,
    status: u8,
    visible_from: String,
    delete_on: Option<String>,
    created_at: String

}

impl ChannelFeedPublication {
    pub fn new(
        id: Option<i64>,
        channel_id: i64,
        application_user_channel_administrator_id: i64,
        content_type: u8,
        content_type_component: String,
        content_type_component_preview: String,
        public_marks_quantity: i64,
        hidden_marks_quantity: i64,
        reactions_quantity: i64,
        viewing_quantity: i64,
        status: u8,
        visible_from: String,
        delete_on: Option<String>,
        created_at: String
    ) -> Self {
        return Self {
            id,
            channel_id,
            application_user_channel_administrator_id,
            content_type,
            content_type_component,
            content_type_component_preview,
            public_marks_quantity,
            hidden_marks_quantity,
            reactions_quantity,
            viewing_quantity,
            status,
            visible_from,
            delete_on,
            created_at
        };
    }

    pub fn get_id<'a>(
        &'a self
    ) -> &'a Option<i64> {
        return &self.id;
    }

    pub fn get_channel_id<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.channel_id;
    }

    pub fn get_application_user_channel_administrator_id<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.application_user_channel_administrator_id;
    }

    pub fn get_content_type<'a>(
        &'a self
    ) -> &'a u8 {
        return &self.content_type;
    }

    pub fn get_content_type_component<'a>(
        &'a self
    ) -> &'a str {
        return self.content_type_component.as_str();
    }

    pub fn get_content_type_component_preview<'a>(
        &'a self
    ) -> &'a str {
        return self.content_type_component_preview.as_str();
    }

    pub fn get_public_marks_quantoty<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.public_marks_quantity;
    }

    pub fn get_hidden_marks_quantity<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.hidden_marks_quantity;
    }

    pub fn get_reactions_quantity<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.reactions_quantity;
    }

    pub fn get_viewing_quantity<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.viewing_quantity;
    }

    pub fn get_status<'a>(
        &'a self
    ) -> &'a u8 {
        return &self.status;
    }

    pub fn get_visible_from<'a>(
        &'a self
    ) -> &'a str {
        return self.visible_from.as_str();
    }

    pub fn get_delete_on<'a>(
        &'a self
    ) -> Option<&'a str> {
        match self.delete_on {
            Some(ref delete_on) => {
                return Some(delete_on.as_str());
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }
}