use crate::domain_layer::entity::entity::application_user_channel_administrator::_component::id::Id as ApplicationUserChannelAdministratorId;
use crate::domain_layer::entity::entity::channel::_component::id::Id as ChannelId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::content_type_component::ContentTypeComponent;
use super::_component::content_type::ContentType;
use super::_component::created_at::CreatedAt;
use super::_component::delete_on::DeleteOn;
use super::_component::id::Id;
use super::_component::status::Status;
use super::_component::viewing_quantity::ViewingQuantity;
use super::_component::visible_from::VisibleFrom;

pub struct ChannelDirectMessagePublication {
    id: Option<Id>,
    channel_id: ChannelId,
    author_application_user_channel_administrator_id: ApplicationUserChannelAdministratorId,
    content_type: ContentType,
    content_type_component: ContentTypeComponent,
    viewing_quantity: ViewingQuantity,
    status: Status,
    visible_from: VisibleFrom,
    delete_on: DeleteOn,
    created_at: CreatedAt
}

impl ChannelDirectMessagePublication {
    pub fn new(
        id: Option<Id>,
        channel_id: ChannelId,
        author_application_user_channel_administrator_id: ApplicationUserChannelAdministratorId,
        content_type: ContentType,
        content_type_component: ContentTypeComponent,
        viewing_quantity: ViewingQuantity,
        status: Status,
        visible_from: VisibleFrom,
        delete_on: DeleteOn,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            id,
            channel_id,
            author_application_user_channel_administrator_id,
            content_type,
            content_type_component,
            viewing_quantity,
            status,
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

    pub fn get_content_type<'this>(&'this self) -> &'this ContentType {
        return &self.content_type;
    }

    pub fn get_content_type_component<'this>(&'this self) -> &'this ContentTypeComponent {
        return &self.content_type_component;
    }

    pub fn get_viewing_quantity<'this>(&'this self) -> &'this ViewingQuantity {
        return &self.viewing_quantity;
    }

    pub fn get_status<'this>(&'this self) -> &'this Status {
        return &self.status;
    }

    pub fn get_visible_from<'this>(&'this self) -> &'this VisibleFrom {
        return &self.visible_from;
    }

    pub fn get_delete_on<'this>(&'this self) -> &'this DeleteOn {
        return &self.delete_on;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}