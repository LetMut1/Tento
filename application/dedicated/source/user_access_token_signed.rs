#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct UserAccessTokenSigned<'a> {
    pub user_access_token__id: &'a str,
    pub user__id: i64,
    pub user_device__id: &'a str,
    pub user_access_token__expires_at: i64,
    // The bitcode(=0.6.3)::Decode not implemented for &'a [u8].
    pub signature: Vec<u8>,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct UserAccessTokenSigned_ {
    pub user_access_token__id: String,
    pub user__id: i64,
    pub user_device__id: String,
    pub user_access_token__expires_at: i64,
    pub signature: Vec<u8>,
}
