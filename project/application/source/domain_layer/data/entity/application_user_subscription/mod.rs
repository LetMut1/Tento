pub struct ApplicationUserSubscription {
    id: Option<i64>,
    publisher_application_user_id: i64,
    subscriber_application_user_id: i64
}

impl ApplicationUserSubscription {
    pub fn new(
        id: Option<i64>,
        publisher_application_user_id: i64,
        subscriber_application_user_id: i64
    ) -> Self {
        return Self {
            id,
            publisher_application_user_id,
            subscriber_application_user_id
        };
    }

    pub fn get_id<'a>(
        &'a self
    ) -> Option<i64> {
        return self.id;
    }

    pub fn get_publisher_application_user_id<'a>(
        &'a self
    ) -> i64 {
        return self.publisher_application_user_id;
    }

    pub fn get_subscriber_application_user_id<'a>(
        &'a self
    ) -> i64 {
        return self.subscriber_application_user_id;
    }
}