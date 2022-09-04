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
pub struct BotResponse {
    #[serde(rename = "bot")]
    pub bot: Box<crate::models::BotResponseBot>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::BotResponseUser>,
}

impl BotResponse {
    pub fn new(bot: crate::models::BotResponseBot, user: crate::models::BotResponseUser) -> BotResponse {
        BotResponse {
            bot: Box::new(bot),
            user: Box::new(user),
        }
    }
}


