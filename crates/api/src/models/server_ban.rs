/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ServerBan : Representation of a server ban on Revolt



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerBan {
    #[serde(rename = "_id")]
    pub _id: Box<crate::models::MemberId>,
    /// Reason for ban creation
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ServerBan {
    /// Representation of a server ban on Revolt
    pub fn new(_id: crate::models::MemberId) -> ServerBan {
        ServerBan {
            _id: Box::new(_id),
            reason: None,
        }
    }
}


