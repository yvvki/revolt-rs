/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ChannelOneOf4 : Voice channel belonging to a server



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelOneOf4 {
    #[serde(rename = "channel_type")]
    pub channel_type: ChannelType,
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Id of the server this channel belongs to
    #[serde(rename = "server")]
    pub server: String,
    /// Display name of the channel
    #[serde(rename = "name")]
    pub name: String,
    /// Channel description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Custom icon attachment
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<crate::models::File>>,
    /// Default permissions assigned to users in this channel
    #[serde(rename = "default_permissions", skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<Box<crate::models::OverrideField>>,
    /// Permissions assigned based on role to this channel
    #[serde(rename = "role_permissions", skip_serializing_if = "Option::is_none")]
    pub role_permissions: Option<::std::collections::HashMap<String, crate::models::OverrideField>>,
    /// Whether this channel is marked as not safe for work
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl ChannelOneOf4 {
    /// Voice channel belonging to a server
    pub fn new(channel_type: ChannelType, _id: String, server: String, name: String) -> ChannelOneOf4 {
        ChannelOneOf4 {
            channel_type,
            _id,
            server,
            name,
            description: None,
            icon: None,
            default_permissions: None,
            role_permissions: None,
            nsfw: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "VoiceChannel")]
    VoiceChannel,
}

impl Default for ChannelType {
    fn default() -> ChannelType {
        Self::VoiceChannel
    }
}

