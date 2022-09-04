/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ChannelOneOf : Personal \"Saved Notes\" channel which allows users to save messages



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelOneOf {
    #[serde(rename = "channel_type")]
    pub channel_type: ChannelType,
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Id of the user this channel belongs to
    #[serde(rename = "user")]
    pub user: String,
}

impl ChannelOneOf {
    /// Personal \"Saved Notes\" channel which allows users to save messages
    pub fn new(channel_type: ChannelType, _id: String, user: String) -> ChannelOneOf {
        ChannelOneOf {
            channel_type,
            _id,
            user,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "SavedMessages")]
    SavedMessages,
}

impl Default for ChannelType {
    fn default() -> ChannelType {
        Self::SavedMessages
    }
}

