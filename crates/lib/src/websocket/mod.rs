use crate::Client;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Debug)]
pub enum Error {
    ConfigurationError,
    TungsteniteError(tokio_tungstenite::tungstenite::Error),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::TungsteniteError(error)
    }
}

pub type WebSocketClient = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub async fn connect(client: &Client) -> Result<WebSocketClient, Error> {
    let url = &client
        .revolt_config
        .as_ref()
        .ok_or(Error::ConfigurationError)?
        .ws;

    let (mut socket, response) = connect_async(url).await?;

    todo!("Implement https://github.com/revoltchat/revolt.js/blob/master/src/websocket/client.ts");

    Ok(socket)
}
