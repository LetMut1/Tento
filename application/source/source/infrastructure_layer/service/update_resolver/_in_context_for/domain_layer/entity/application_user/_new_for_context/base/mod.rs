use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;

pub struct Base {
    update_email: bool,
    update_nickname: bool,
    update_password_hash: bool
}

impl Base {
    pub fn new(
        update_email: bool,
        update_nickname: bool,
        update_password_hash: bool
    ) -> Self {
        return Self {
            update_email,
            update_nickname,
            update_password_hash
        };
    }
}

impl UpdateResolverApplicationUserTrait for Base {
    fn is_update_email<'this>(
        &'this self
    ) -> bool {
        return self.update_email;
    }

    fn is_update_nickname<'this>(
        &'this self
    ) -> bool {
        return self.update_nickname;
    }

    fn is_update_password_hash<'this>(
        &'this self
    ) -> bool {
        return self.update_password_hash;
    }
}