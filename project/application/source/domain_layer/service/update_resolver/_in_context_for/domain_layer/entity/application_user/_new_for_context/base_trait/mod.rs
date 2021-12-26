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
}