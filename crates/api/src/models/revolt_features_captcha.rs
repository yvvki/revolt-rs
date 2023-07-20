/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// RevoltFeaturesCaptcha : hCaptcha configuration



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RevoltFeaturesCaptcha {
    /// Whether captcha is enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Client key used for solving captcha
    #[serde(rename = "key")]
    pub key: String,
}

impl RevoltFeaturesCaptcha {
    /// hCaptcha configuration
    pub fn new(enabled: bool, key: String) -> RevoltFeaturesCaptcha {
        RevoltFeaturesCaptcha {
            enabled,
            key,
        }
    }
}


