// There is not UserAccessRefreshTokenSigned(pub &'a [u8]), because Bitcode::Decode not implemented
// for &'a [u8]. Maybe, it will be implemented in a future releases.
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct UserAccessRefreshTokenSigned(pub Vec<u8>);
