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
pub struct DataDefaultChannelPermissions {
    /// Allow / deny values to set for members in this `TextChannel` or `VoiceChannel`
    #[serde(rename = "permissions")]
    pub permissions: Option<Box<crate::models::ModelOverride>>,
}

impl DataDefaultChannelPermissions {
    pub fn new(permissions: Option<crate::models::ModelOverride>) -> DataDefaultChannelPermissions {
        DataDefaultChannelPermissions {
            permissions: Box::new(permissions),
        }
    }
}

