#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChannelTokenHashed {
    pub channel_token__expires_at: i64,
    pub hash: u64,
}
