pub struct Base {
    update_wrong_enter_tries_quantity: bool,
    update_created_at: bool
}

impl Base {
    pub fn new(
        update_wrong_enter_tries_quantity: bool,
        update_created_at: bool
    ) -> Self {
        return Self {
            update_created_at,
            update_wrong_enter_tries_quantity
        };
    }

    pub fn is_update_wrong_enter_tries_quantity<'a>(
        &'a self
    ) -> bool {
        return self.update_wrong_enter_tries_quantity;
    }

    pub fn is_update_created_at<'a>(
        &'a self
    ) -> bool {
        return self.update_created_at;
    }
}