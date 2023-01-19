pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
    // pub send_tx: UnboundedSender<Message>,
    // send_rx: UnboundedReceiver<Message>,
    // pub recv_rx: UnboundedReceiver<Message>,
    // recv_tx: UnboundedSender<Message>,
    // pub runner: Box<dyn Future<Output = ()> + 'a>,
    // pub runner: Box<dyn Fn() -> () + 'a>,
    pub reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new(sender: UnboundedReceiver<Message>) -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        let (ws_stream, _) = connect_async(url).await.expect("failed to connect");
        let (write, reader) = ws_stream.split();
        let client_to_ws = sender.map(Ok).forward(write);
        tokio::spawn(client_to_ws);
        Ok(Self {
            protocol,
            server,
            port,
            reader,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod test {
    use crate::client::ChatinoClient;
    use anyhow::Result;
    use futures_util::StreamExt;
    use log::{error, info};
    use std::time::Duration;
    use tokio::time::sleep;
    use tokio_tungstenite::tungstenite::Message;

    #[test]
    fn test_client() -> Result<()> {
        tracing_subscriber::fmt::init();
        let (sender, send_rx) = futures_channel::mpsc::unbounded();
        let runner = async {
            let r = ChatinoClient::new(send_rx).await.unwrap();
            let reader = r.reader;
            let ws_to_client = {
                reader.for_each(|message| async {
                    match message {
                        Ok(message) => {
                            println!("recv message: {}", message)
                        }
                        Err(e) => {
                            error!("err on connection: {}", e)
                        }
                    };
                })
            };
            tokio::spawn(ws_to_client);
            loop {
                sleep(Duration::from_millis(1000)).await;
                match sender.unbounded_send(Message::Text("{\"cmd\":\"getinfo\"}".to_string())) {
                    Ok(_) => {
                        info!("message sent")
                    }
                    Err(e) => {
                        error!("sender error: {}", e)
                    }
                }
            }
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(runner);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl ChatinoClient {}
