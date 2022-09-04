/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.5.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SpecialOneOf3 : Lightspeed.tv stream



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecialOneOf3 {
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "content_type")]
    pub content_type: crate::models::LightspeedType,
    #[serde(rename = "id")]
    pub id: String,
}

impl SpecialOneOf3 {
    /// Lightspeed.tv stream
    pub fn new(_type: Type, content_type: crate::models::LightspeedType, id: String) -> SpecialOneOf3 {
        SpecialOneOf3 {
            _type,
            content_type,
            id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Lightspeed")]
    Lightspeed,
}

impl Default for Type {
    fn default() -> Type {
        Self::Lightspeed
    }
}

