pub trait BaseTrait {
    fn is_update_email<'this>(
        &'this self
    ) -> bool;

    fn is_update_nickname<'this>(
        &'this self
    ) -> bool;

    fn is_update_password_hash<'this>(
        &'this self
    ) -> bool;

    fn should_update<'this>(
        &'this self
    ) -> bool {
        return self.is_update_email() || self.is_update_nickname() || self.is_update_password_hash();
    }
}