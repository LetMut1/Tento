use crate::domain_layer::entity::entity::application_user_channel_administrator::_component::id::Id as ApplicationUserChannelAdministratorId;
use crate::domain_layer::entity::entity::channel::_component::id::Id as ChannelId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::content_type_component_preview::ContentTypeComponentPreview;
use super::_component::content_type_component::ContentTypeComponent;
use super::_component::content_type::ContentType;
use super::_component::created_at::CreatedAt;
use super::_component::delete_on::DeleteOn;
use super::_component::hidden_marks_quantity::HiddenMarksQuantity;
use super::_component::id::Id;
use super::_component::is_entertaining::IsEntertaining;
use super::_component::public_marks_quantity::PublicMarksQuantity;
use super::_component::reactions_quantity::ReactionsQuantity;
use super::_component::viewing_quantity::ViewingQuantity;
use super::_component::visible_from::VisibleFrom;

pub struct ChannelFeedPublication {
    id: Option<Id>,
    channel_id: ChannelId,
    author_application_user_channel_administrator_id: ApplicationUserChannelAdministratorId,
    is_entertaining: IsEntertaining,
    content_type: ContentType,
    content_type_component: ContentTypeComponent,
    content_type_component_preview: ContentTypeComponentPreview,
    public_marks_quantity: PublicMarksQuantity,
    hidden_marks_quantity: HiddenMarksQuantity,
    reactions_quantity: ReactionsQuantity,
    viewing_quantity: ViewingQuantity,
    visible_from: VisibleFrom,
    delete_on: Option<DeleteOn>,
    created_at: CreatedAt

}

impl ChannelFeedPublication {
    pub fn new(
        id: Option<Id>,
        channel_id: ChannelId,
        author_application_user_channel_administrator_id: ApplicationUserChannelAdministratorId,
        is_entertaining: IsEntertaining,
        content_type: ContentType,
        content_type_component: ContentTypeComponent,
        content_type_component_preview: ContentTypeComponentPreview,
        public_marks_quantity: PublicMarksQuantity,
        hidden_marks_quantity: HiddenMarksQuantity,
        reactions_quantity: ReactionsQuantity,
        viewing_quantity: ViewingQuantity,
        visible_from: VisibleFrom,
        delete_on: Option<DeleteOn>,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            id,
            channel_id,
            author_application_user_channel_administrator_id,
            is_entertaining,
            content_type,
            content_type_component,
            content_type_component_preview,
            public_marks_quantity,
            hidden_marks_quantity,
            reactions_quantity,
            viewing_quantity,
            visible_from,
            delete_on,
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

    pub fn get_channel_id<'this>(&'this self) -> &'this ChannelId {
        return &self.channel_id;
    }

    pub fn get_author_application_user_channel_administrator_id<'this>(&'this self) -> &'this ApplicationUserChannelAdministratorId {
        return &self.author_application_user_channel_administrator_id;
    }

    pub fn get_is_entertaining<'this>(&'this self) -> &'this IsEntertaining {
        return &self.is_entertaining;
    }

    pub fn get_content_type<'this>(&'this self) -> &'this ContentType {
        return &self.content_type;
    }

    pub fn get_content_type_component<'this>(&'this self) -> &'this ContentTypeComponent {
        return &self.content_type_component;
    }

    pub fn get_content_type_component_preview<'this>(&'this self) -> &'this ContentTypeComponentPreview {
        return &self.content_type_component_preview;
    }

    pub fn get_public_marks_quantoty<'this>(&'this self) -> &'this PublicMarksQuantity {
        return &self.public_marks_quantity;
    }

    pub fn get_hidden_marks_quantity<'this>(&'this self) -> &'this HiddenMarksQuantity {
        return &self.hidden_marks_quantity;
    }

    pub fn get_reactions_quantity<'this>(&'this self) -> &'this ReactionsQuantity {
        return &self.reactions_quantity;
    }

    pub fn get_viewing_quantity<'this>(&'this self) -> &'this ViewingQuantity {
        return &self.viewing_quantity;
    }

    pub fn get_visible_from<'this>(&'this self) -> &'this VisibleFrom {
        return &self.visible_from;
    }

    pub fn get_delete_on<'this>(&'this self) -> &'this Option<DeleteOn> {
        return &self.delete_on;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}