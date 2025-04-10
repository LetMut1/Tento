#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct UserAccessRefreshTokenSigned {
    pub user_access_refresh_token__expires_at: i64,
    // The bitcode(=0.6.3)::Decode not implemented for &'a [u8]
    pub signature: Vec<u8>,
}
