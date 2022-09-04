/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RevoltFeatures {
    #[serde(rename = "captcha")]
    pub captcha: Box<crate::models::RevoltFeaturesCaptcha>,
    /// Whether email verification is enabled
    #[serde(rename = "email")]
    pub email: bool,
    /// Whether this server is invite only
    #[serde(rename = "invite_only")]
    pub invite_only: bool,
    #[serde(rename = "autumn")]
    pub autumn: Box<crate::models::RevoltFeaturesAutumn>,
    #[serde(rename = "january")]
    pub january: Box<crate::models::RevoltFeaturesJanuary>,
    #[serde(rename = "voso")]
    pub voso: Box<crate::models::RevoltFeaturesVoso>,
}

impl RevoltFeatures {
    pub fn new(captcha: crate::models::RevoltFeaturesCaptcha, email: bool, invite_only: bool, autumn: crate::models::RevoltFeaturesAutumn, january: crate::models::RevoltFeaturesJanuary, voso: crate::models::RevoltFeaturesVoso) -> RevoltFeatures {
        RevoltFeatures {
            captcha: Box::new(captcha),
            email,
            invite_only,
            autumn: Box::new(autumn),
            january: Box::new(january),
            voso: Box::new(voso),
        }
    }
}

