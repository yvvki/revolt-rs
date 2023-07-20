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
pub struct DataDefaultChannelPermissionsAnyOf1 {
    #[serde(rename = "permissions")]
    pub permissions: Box<crate::models::DataDefaultChannelPermissionsAnyOf1Permissions>,
}

impl DataDefaultChannelPermissionsAnyOf1 {
    pub fn new(permissions: crate::models::DataDefaultChannelPermissionsAnyOf1Permissions) -> DataDefaultChannelPermissionsAnyOf1 {
        DataDefaultChannelPermissionsAnyOf1 {
            permissions: Box::new(permissions),
        }
    }
}


