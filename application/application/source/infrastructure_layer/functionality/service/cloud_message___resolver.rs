use super::resolver::Resolver;

pub use crate::infrastructure_layer::data::control_type_registry::CloudMessage;

impl Resolver<CloudMessage> {
    pub fn deauthorize_application_user_from_all_devices() -> () {
        return (); // TODO stub
    }
}
