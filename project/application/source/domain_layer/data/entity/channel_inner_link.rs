pub struct ChannelInnerLink {
    /// application_user_id
    from: i64,
    /// application_user_id
    to: i64,
    created_at: String
}

impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;

    pub fn new(
        from: i64,
        to: i64,
        created_at: String
    ) -> Self {
        return Self {
            from,
            to,
            created_at
        };
    }
}