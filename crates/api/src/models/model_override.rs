/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Override : Representation of a single permission override



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Override {
    /// Allow bit flags
    #[serde(rename = "allow")]
    pub allow: i32,
    /// Disallow bit flags
    #[serde(rename = "deny")]
    pub deny: i32,
}

impl Override {
    /// Representation of a single permission override
    pub fn new(allow: i32, deny: i32) -> Override {
        Override {
            allow,
            deny,
        }
    }
}


