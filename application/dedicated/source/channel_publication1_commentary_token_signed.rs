#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChannelPublication1CommentaryTokenSigned {
    pub channel_publication1_commentary__id: i64,
    pub channel_publication1_commentary_token__obfuscation_value: i64,
    pub channel_publication1_commentary_token__expires_at: i64,
    pub channel_publication1_commentary_token__commentary_author: i64,
    // The bitcode(=0.6.3)::Decode not implemented for &'a [u8].
    pub signature: Vec<u8>,
}
