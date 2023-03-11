pub struct ChannelOuterLink {
    /// application_user_id
    from: i64,
    alias: String,
    adress: String,
    created_at: String
}

impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;
}