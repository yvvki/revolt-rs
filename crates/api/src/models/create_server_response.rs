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
pub struct CreateServerResponse {
    #[serde(rename = "server")]
    pub server: Box<crate::models::CreateServerResponseServer>,
    /// Default channels
    #[serde(rename = "channels")]
    pub channels: Vec<crate::models::Channel>,
}

impl CreateServerResponse {
    pub fn new(server: crate::models::CreateServerResponseServer, channels: Vec<crate::models::Channel>) -> CreateServerResponse {
        CreateServerResponse {
            server: Box::new(server),
            channels,
        }
    }
}


