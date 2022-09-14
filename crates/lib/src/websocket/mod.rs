use crate::Client;

mod error;
pub use error::Error;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub type WebSocketClient = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub async fn connect(client: &Client) -> Result<WebSocketClient, Error> {
    let url = &client
        .revolt_config
        .as_ref()
        .ok_or(Error::ConfigurationError)?
        .ws;

    let session = client
        .session
        .as_ref()
        .ok_or(Error::SessionError)?;

    let (mut socket, response) = connect_async(url).await?;

    todo!("Implement https://github.com/revoltchat/revolt.js/blob/master/src/websocket/client.ts");

    Ok(socket)
}
