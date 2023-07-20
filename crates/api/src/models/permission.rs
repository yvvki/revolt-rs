/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Permission : Permission value on Revolt  This should be restricted to the lower 52 bits to prevent any potential issues with Javascript. Also leave empty spaces for future permission flags to be added.

/// Permission value on Revolt  This should be restricted to the lower 52 bits to prevent any potential issues with Javascript. Also leave empty spaces for future permission flags to be added.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "ManageChannel")]
    ManageChannel,
    #[serde(rename = "ManageServer")]
    ManageServer,
    #[serde(rename = "ManagePermissions")]
    ManagePermissions,
    #[serde(rename = "ManageRole")]
    ManageRole,
    #[serde(rename = "ManageCustomisation")]
    ManageCustomisation,
    #[serde(rename = "KickMembers")]
    KickMembers,
    #[serde(rename = "BanMembers")]
    BanMembers,
    #[serde(rename = "TimeoutMembers")]
    TimeoutMembers,
    #[serde(rename = "AssignRoles")]
    AssignRoles,
    #[serde(rename = "ChangeNickname")]
    ChangeNickname,
    #[serde(rename = "ManageNicknames")]
    ManageNicknames,
    #[serde(rename = "ChangeAvatar")]
    ChangeAvatar,
    #[serde(rename = "RemoveAvatars")]
    RemoveAvatars,
    #[serde(rename = "ViewChannel")]
    ViewChannel,
    #[serde(rename = "ReadMessageHistory")]
    ReadMessageHistory,
    #[serde(rename = "SendMessage")]
    SendMessage,
    #[serde(rename = "ManageMessages")]
    ManageMessages,
    #[serde(rename = "ManageWebhooks")]
    ManageWebhooks,
    #[serde(rename = "InviteOthers")]
    InviteOthers,
    #[serde(rename = "SendEmbeds")]
    SendEmbeds,
    #[serde(rename = "UploadFiles")]
    UploadFiles,
    #[serde(rename = "Masquerade")]
    Masquerade,
    #[serde(rename = "React")]
    React,
    #[serde(rename = "Connect")]
    Connect,
    #[serde(rename = "Speak")]
    Speak,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "MuteMembers")]
    MuteMembers,
    #[serde(rename = "DeafenMembers")]
    DeafenMembers,
    #[serde(rename = "MoveMembers")]
    MoveMembers,
    #[serde(rename = "GrantAllSafe")]
    GrantAllSafe,
    #[serde(rename = "GrantAll")]
    GrantAll,

}

impl ToString for Permission {
    fn to_string(&self) -> String {
        match self {
            Self::ManageChannel => String::from("ManageChannel"),
            Self::ManageServer => String::from("ManageServer"),
            Self::ManagePermissions => String::from("ManagePermissions"),
            Self::ManageRole => String::from("ManageRole"),
            Self::ManageCustomisation => String::from("ManageCustomisation"),
            Self::KickMembers => String::from("KickMembers"),
            Self::BanMembers => String::from("BanMembers"),
            Self::TimeoutMembers => String::from("TimeoutMembers"),
            Self::AssignRoles => String::from("AssignRoles"),
            Self::ChangeNickname => String::from("ChangeNickname"),
            Self::ManageNicknames => String::from("ManageNicknames"),
            Self::ChangeAvatar => String::from("ChangeAvatar"),
            Self::RemoveAvatars => String::from("RemoveAvatars"),
            Self::ViewChannel => String::from("ViewChannel"),
            Self::ReadMessageHistory => String::from("ReadMessageHistory"),
            Self::SendMessage => String::from("SendMessage"),
            Self::ManageMessages => String::from("ManageMessages"),
            Self::ManageWebhooks => String::from("ManageWebhooks"),
            Self::InviteOthers => String::from("InviteOthers"),
            Self::SendEmbeds => String::from("SendEmbeds"),
            Self::UploadFiles => String::from("UploadFiles"),
            Self::Masquerade => String::from("Masquerade"),
            Self::React => String::from("React"),
            Self::Connect => String::from("Connect"),
            Self::Speak => String::from("Speak"),
            Self::Video => String::from("Video"),
            Self::MuteMembers => String::from("MuteMembers"),
            Self::DeafenMembers => String::from("DeafenMembers"),
            Self::MoveMembers => String::from("MoveMembers"),
            Self::GrantAllSafe => String::from("GrantAllSafe"),
            Self::GrantAll => String::from("GrantAll"),
        }
    }
}

impl Default for Permission {
    fn default() -> Permission {
        Self::ManageChannel
    }
}




