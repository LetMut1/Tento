use super::entity_workflow_event::entity_workflow_event::EntityWorkflowEvent;

pub enum ActionHandlerResult<S> {
    EntityWorkflowEvent {
        entity_workflow_event: EntityWorkflowEvent
    },
    ActionHandlerOutcomingData {
        action_handler_outcoming_data: S
    }
}