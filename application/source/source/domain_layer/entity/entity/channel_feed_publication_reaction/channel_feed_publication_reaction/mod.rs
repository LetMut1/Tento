use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::channel_feed_publication::_component::id::Id as ChannelFeedPublicationId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::content_type_component::ContentTypeComponent;
use super::_component::content_type::ContentType;
use super::_component::created_at::CreatedAt;
use super::_component::id::Id;
use super::_component::public_marks_quantity::PublicMarksQuantity;

pub struct ChannelFeedPublicationReaction {
    id: Option<Id>,
    channel_feed_publication_id: ChannelFeedPublicationId,
    application_user_id: ApplicationUserId,
    content_type: ContentType,
    content_type_component: ContentTypeComponent,
    public_marks_quantity: PublicMarksQuantity,
    created_at: CreatedAt
}

impl ChannelFeedPublicationReaction {
    pub fn new(
        id: Option<Id>,
        channel_feed_publication_id: ChannelFeedPublicationId,
        application_user_id: ApplicationUserId,
        content_type: ContentType,
        content_type_component: ContentTypeComponent,
        public_marks_quantity: PublicMarksQuantity,
        created_at: CreatedAt
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

    pub fn get_id<'this>(&'this self) -> Result<&'this Id, BaseError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_channel_feed_publication_id<'this>(&'this self) -> &'this ChannelFeedPublicationId {
        return &self.channel_feed_publication_id;
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.application_user_id;
    }

    pub fn get_content_type<'this>(&'this self) -> &'this ContentType {
        return &self.content_type;
    }

    pub fn get_content_type_component<'this>(&'this self) -> &'this ContentTypeComponent {
        return &self.content_type_component;
    }

    pub fn get_public_marks_quantoty<'this>(&'this self) -> &'this PublicMarksQuantity {
        return &self.public_marks_quantity;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}