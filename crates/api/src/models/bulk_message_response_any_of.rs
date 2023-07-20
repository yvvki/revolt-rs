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
pub struct BulkMessageResponseAnyOf {
    /// List of messages
    #[serde(rename = "messages")]
    pub messages: Vec<crate::models::Message>,
    /// List of users
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
    /// List of members
    #[serde(rename = "members", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub members: Option<Option<Vec<crate::models::Member>>>,
}

impl BulkMessageResponseAnyOf {
    pub fn new(messages: Vec<crate::models::Message>, users: Vec<crate::models::User>) -> BulkMessageResponseAnyOf {
        BulkMessageResponseAnyOf {
            messages,
            users,
            members: None,
        }
    }
}


