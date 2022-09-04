/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SystemMessageChannels : System message channel assignments



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SystemMessageChannels {
    /// ID of channel to send user join messages in
    #[serde(rename = "user_joined", skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<String>,
    /// ID of channel to send user left messages in
    #[serde(rename = "user_left", skip_serializing_if = "Option::is_none")]
    pub user_left: Option<String>,
    /// ID of channel to send user kicked messages in
    #[serde(rename = "user_kicked", skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<String>,
    /// ID of channel to send user banned messages in
    #[serde(rename = "user_banned", skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<String>,
}

impl SystemMessageChannels {
    /// System message channel assignments
    pub fn new() -> SystemMessageChannels {
        SystemMessageChannels {
            user_joined: None,
            user_left: None,
            user_kicked: None,
            user_banned: None,
        }
    }
}


