/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ChannelUnreadId : Composite key pointing to a user's view of a channel



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelUnreadId {
    /// Channel Id
    #[serde(rename = "channel")]
    pub channel: String,
    /// User Id
    #[serde(rename = "user")]
    pub user: String,
}

impl ChannelUnreadId {
    /// Composite key pointing to a user's view of a channel
    pub fn new(channel: String, user: String) -> ChannelUnreadId {
        ChannelUnreadId {
            channel,
            user,
        }
    }
}


