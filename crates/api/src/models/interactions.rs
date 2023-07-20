/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Interactions : Information to guide interactions on this message



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Interactions {
    /// Reactions which should always appear and be distinct
    #[serde(rename = "reactions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Option<Vec<String>>>,
    /// Whether reactions should be restricted to the given list  Can only be set to true if reactions list is of at least length 1
    #[serde(rename = "restrict_reactions", skip_serializing_if = "Option::is_none")]
    pub restrict_reactions: Option<bool>,
}

impl Interactions {
    /// Information to guide interactions on this message
    pub fn new() -> Interactions {
        Interactions {
            reactions: None,
            restrict_reactions: None,
        }
    }
}


