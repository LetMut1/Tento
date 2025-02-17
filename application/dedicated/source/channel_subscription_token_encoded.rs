#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(Clone, Copy, bitcode::Encode, bitcode::Decode)]
pub struct ChannelSubscriptionTokenEncoded(pub u64);
