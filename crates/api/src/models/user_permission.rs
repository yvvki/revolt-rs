/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// UserPermission : User permission definitions

/// User permission definitions
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserPermission {
    #[serde(rename = "Access")]
    Access,
    #[serde(rename = "ViewProfile")]
    ViewProfile,
    #[serde(rename = "SendMessage")]
    SendMessage,
    #[serde(rename = "Invite")]
    Invite,

}

impl ToString for UserPermission {
    fn to_string(&self) -> String {
        match self {
            Self::Access => String::from("Access"),
            Self::ViewProfile => String::from("ViewProfile"),
            Self::SendMessage => String::from("SendMessage"),
            Self::Invite => String::from("Invite"),
        }
    }
}

impl Default for UserPermission {
    fn default() -> UserPermission {
        Self::Access
    }
}




