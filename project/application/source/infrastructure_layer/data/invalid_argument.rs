use crate::domain_layer::data::entity_invalid_argument::EntityInvalidArgument;

pub enum InvalidArgument {
    Entity {
        entity_invalid_argument: EntityInvalidArgument
    },
    Common {
        common_invalid_argument: CommomInvalidArgument
    }
}

pub enum CommomInvalidArgument {
    HttpHeaders
}