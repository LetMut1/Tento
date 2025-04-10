#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChannelTokenSigned {
    pub channel__id: i64,
    pub channel_token__obfuscation_value: i64,
    pub channel_token__expires_at: i64,
    pub channel_token__is_channel_subscription_exist: bool,
    // The bitcode(=0.6.3)::Decode not implemented for &'a [u8].
    pub signature: Vec<u8>,
}
