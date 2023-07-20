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
pub struct SystemMessageOneOf3 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "id")]
    pub id: String,
}

impl SystemMessageOneOf3 {
    pub fn new(r#type: RHashType, id: String) -> SystemMessageOneOf3 {
        SystemMessageOneOf3 {
            r#type,
            id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user_joined")]
    UserJoined,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::UserJoined
    }
}

