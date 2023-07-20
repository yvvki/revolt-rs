/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MfaResponseAnyOf2 {
    #[serde(rename = "totp_code")]
    pub totp_code: String,
}

impl MfaResponseAnyOf2 {
    pub fn new(totp_code: String) -> MfaResponseAnyOf2 {
        MfaResponseAnyOf2 {
            totp_code,
        }
    }
}


