use crate::{
    apis::{self, core_api::*},
    models::{self}
};

#[derive(Debug, Default, Clone)]
pub struct Client {
    pub configuration: apis::configuration::Configuration,

    pub revolt_config: Option<revolt_api::models::RevoltConfig>,

    pub session: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetches the configuration of the server.
    /// 
    /// Unlike [fetch_configuration], this function also fetches the
    /// configuration if it has already been fethced before.
    pub async fn connect(&mut self)
    -> Result<(), apis::Error<RootRootError>>
    {
        let revolt_config = root_root(&self.configuration).await?;

        self.revolt_config = Some(revolt_config);

        Ok(())
    }

    /// Fetches the configuration of the server if it has not been already
    /// fetched.
    pub async fn fetch_configuration(&mut self)
    -> Result<(), apis::Error<RootRootError>>
    {
        if let None = self.revolt_config {
            self.connect().await?
        }

        Ok(())
    }

    /// Update API object to use authentication.
    fn update_headers(&mut self) {
        self.configuration.basic_auth = Some((
            String::new(),
            self.session.clone(),
        ));
    }

    /// Log in as a bot using a bot token.
    pub async fn login_bot(&mut self, token: String)
    -> Result<(), apis::Error<RootRootError>>
    {
        self.fetch_configuration().await?;

        self.session = Some(token);

        self.update_headers();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_configuration() {
        let mut client = Client::new();

        let result = client.fetch_configuration().await.unwrap();

        assert_eq!(result, ());
    }
}
