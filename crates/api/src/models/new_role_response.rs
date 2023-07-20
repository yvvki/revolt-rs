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
pub struct NewRoleResponse {
    /// Id of the role
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "role")]
    pub role: Box<crate::models::NewRoleResponseRole>,
}

impl NewRoleResponse {
    pub fn new(id: String, role: crate::models::NewRoleResponseRole) -> NewRoleResponse {
        NewRoleResponse {
            id,
            role: Box::new(role),
        }
    }
}


