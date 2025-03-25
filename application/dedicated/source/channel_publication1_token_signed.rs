#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChannelPublication1TokenSigned {
    pub channel_publication1__id: i64,
    pub channel_publication1__obfuscation_value: i64,
    pub channel_publication1_token__expires_at: i64,
    // The bitcode(=0.6.3)::Decode not implemented for &'a [u8].
    pub signature: Vec<u8>,
}
