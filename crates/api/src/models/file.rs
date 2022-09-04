/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// File : Representation of a File on Revolt Generated by Autumn



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct File {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Tag / bucket this file was uploaded to
    #[serde(rename = "tag")]
    pub tag: String,
    /// Original filename
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::FileMetadata>,
    /// Raw content type of this file
    #[serde(rename = "content_type")]
    pub content_type: String,
    /// Size of this file (in bytes)
    #[serde(rename = "size")]
    pub size: i32,
    /// Whether this file was deleted
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Whether this file was reported
    #[serde(rename = "reported", skip_serializing_if = "Option::is_none")]
    pub reported: Option<bool>,
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// Id of the object this file is associated with
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}

impl File {
    /// Representation of a File on Revolt Generated by Autumn
    pub fn new(_id: String, tag: String, filename: String, metadata: crate::models::FileMetadata, content_type: String, size: i32) -> File {
        File {
            _id,
            tag,
            filename,
            metadata: Box::new(metadata),
            content_type,
            size,
            deleted: None,
            reported: None,
            message_id: None,
            user_id: None,
            server_id: None,
            object_id: None,
        }
    }
}


