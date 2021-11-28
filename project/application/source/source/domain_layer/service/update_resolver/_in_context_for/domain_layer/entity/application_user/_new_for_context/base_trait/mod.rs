pub trait BaseTrait {
    fn is_update_email<'a>(
        &'a self
    ) -> bool;

    fn is_update_nickname<'a>(
        &'a self
    ) -> bool;

    fn is_update_password_hash<'a>(
        &'a self
    ) -> bool;

    fn should_update<'a>(
        &'a self
    ) -> bool {
        return self.is_update_email() || self.is_update_nickname() || self.is_update_password_hash();
    }
}