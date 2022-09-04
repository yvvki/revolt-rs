/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RAuthErrorOneOf6 {
    #[serde(rename = "type")]
    pub _type: Type,
}

impl RAuthErrorOneOf6 {
    pub fn new(_type: Type) -> RAuthErrorOneOf6 {
        RAuthErrorOneOf6 {
            _type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "EmailFailed")]
    EmailFailed,
}

impl Default for Type {
    fn default() -> Type {
        Self::EmailFailed
    }
}

