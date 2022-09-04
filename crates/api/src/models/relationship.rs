/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Relationship : Relationship entry indicating current status with other user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "status")]
    pub status: crate::models::RelationshipStatus,
}

impl Relationship {
    /// Relationship entry indicating current status with other user
    pub fn new(_id: String, status: crate::models::RelationshipStatus) -> Relationship {
        Relationship {
            _id,
            status,
        }
    }
}


