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
pub struct ErrorOneOf6 {
    #[serde(rename = "type")]
    pub _type: Type,
}

impl ErrorOneOf6 {
    pub fn new(_type: Type) -> ErrorOneOf6 {
        ErrorOneOf6 {
            _type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AlreadySentRequest")]
    AlreadySentRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::AlreadySentRequest
    }
}

