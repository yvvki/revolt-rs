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
pub struct SystemMessageOneOf10 {
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
}

impl SystemMessageOneOf10 {
    pub fn new(_type: Type, from: String, to: String) -> SystemMessageOneOf10 {
        SystemMessageOneOf10 {
            _type,
            from,
            to,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged,
}

impl Default for Type {
    fn default() -> Type {
        Self::ChannelOwnershipChanged
    }
}
