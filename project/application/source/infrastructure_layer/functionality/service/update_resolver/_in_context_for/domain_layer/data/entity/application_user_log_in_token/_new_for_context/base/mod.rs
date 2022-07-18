pub struct Base {
    update_value: bool,
    update_wrong_enter_tries_quantity: bool,
    update_created_at: bool
}

impl Base {
    pub fn new(
        update_value: bool,
        update_wrong_enter_tries_quantity: bool,
        update_created_at: bool
    ) -> Self {
        return Self {
            update_value,
            update_created_at,
            update_wrong_enter_tries_quantity
        };
    }

    pub fn get_update_value<'a>(
        &'a self
    ) -> bool {
        return self.update_value;
    }

    pub fn get_update_wrong_enter_tries_quantity<'a>(
        &'a self
    ) -> bool {
        return self.update_wrong_enter_tries_quantity;
    }

    pub fn get_update_created_at<'a>(
        &'a self
    ) -> bool {
        return self.update_created_at;
    }
}