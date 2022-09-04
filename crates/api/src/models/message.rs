/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Message : Representation of a Message on Revolt



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Message {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Unique value generated by client sending this message
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Id of the channel this message was sent in
    #[serde(rename = "channel")]
    pub channel: String,
    /// Id of the user that sent this message
    #[serde(rename = "author")]
    pub author: String,
    /// Message content
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Box<crate::models::MessageSystem>>,
    /// Array of attachments
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::File>>,
    /// Time at which this message was last edited
    #[serde(rename = "edited", skip_serializing_if = "Option::is_none")]
    pub edited: Option<Box<String>>,
    /// Attached embeds to this message
    #[serde(rename = "embeds", skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<crate::models::Embed>>,
    /// Array of user ids mentioned in this message
    #[serde(rename = "mentions", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    /// Array of message ids this message is replying to
    #[serde(rename = "replies", skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,
    /// Hashmap of emoji IDs to array of user IDs
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "interactions", skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Box<crate::models::MessageInteractions>>,
    #[serde(rename = "masquerade", skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Box<crate::models::MessageMasquerade>>,
}

impl Message {
    /// Representation of a Message on Revolt
    pub fn new(_id: String, channel: String, author: String) -> Message {
        Message {
            _id,
            nonce: None,
            channel,
            author,
            content: None,
            system: None,
            attachments: None,
            edited: None,
            embeds: None,
            mentions: None,
            replies: None,
            reactions: None,
            interactions: None,
            masquerade: None,
        }
    }
}


