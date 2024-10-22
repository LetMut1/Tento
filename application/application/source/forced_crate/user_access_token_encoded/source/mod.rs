#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct UserAccessTokenEncoded {
    pub serialized: Vec<u8>,
    pub encoded: Vec<u8>,
}
