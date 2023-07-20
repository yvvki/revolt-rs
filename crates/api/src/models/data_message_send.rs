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
pub struct DataMessageSend {
    /// Unique token to prevent duplicate message sending  **This is deprecated and replaced by `Idempotency-Key`!**
    #[serde(rename = "nonce", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<String>>,
    /// Message content to send
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    /// Attachments to include in message
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<String>>>,
    /// Messages to reply to
    #[serde(rename = "replies", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub replies: Option<Option<Vec<crate::models::Reply>>>,
    /// Embeds to include in message  Text embed content contributes to the content length cap
    #[serde(rename = "embeds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<crate::models::SendableEmbed>>>,
    #[serde(rename = "masquerade", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Option<Box<crate::models::DataMessageSendMasquerade>>>,
    #[serde(rename = "interactions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Option<Box<crate::models::DataMessageSendInteractions>>>,
}

impl DataMessageSend {
    pub fn new() -> DataMessageSend {
        DataMessageSend {
            nonce: None,
            content: None,
            attachments: None,
            replies: None,
            embeds: None,
            masquerade: None,
            interactions: None,
        }
    }
}


