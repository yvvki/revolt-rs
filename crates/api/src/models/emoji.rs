/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Emoji : Representation of an Emoji on Revolt



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Emoji {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "parent")]
    pub parent: Box<crate::models::EmojiParent>,
    /// Uploader user id
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    /// Emoji name
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the emoji is animated
    #[serde(rename = "animated", skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    /// Whether the emoji is marked as nsfw
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl Emoji {
    /// Representation of an Emoji on Revolt
    pub fn new(_id: String, parent: crate::models::EmojiParent, creator_id: String, name: String) -> Emoji {
        Emoji {
            _id,
            parent: Box::new(parent),
            creator_id,
            name,
            animated: None,
            nsfw: None,
        }
    }
}


