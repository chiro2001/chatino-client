// pub const WSS_URL: &'static str = "wss://ws.crosst.chat:35197/";
pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use log::error;
// use tokio::sync::mpsc::{channel, Sender, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
// use websockets::Frame;

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
    // pub send_tx: Sender<Result<Frame, &'static str>>,
    // pub recv_tx: Sender<Result<Frame, &'static str>>,
    // pub send_tx: UnboundedSender<String>,
    // pub recv_tx: UnboundedSender<Frame>,
    // pub recv_rx: UnboundedReceiver<String>,
    pub send_tx: UnboundedSender<Message>,
    pub recv_rx: UnboundedReceiver<Message>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new() -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        // let mut ws = websockets::WebSocket::connect(&url).await?;
        // let (send_tx, mut send_rx) = channel(1024);
        // let (recv_tx, recv_rx) = channel(1024);
        let (send_tx, send_rx) = futures_channel::mpsc::unbounded();
        let (recv_tx, recv_rx) = futures_channel::mpsc::unbounded();
        let s = Self {
            protocol,
            server,
            port,
            send_tx,
            recv_rx,
        };
        // ws.send_text("{\"cmd\":\"getinfo\"}".to_string()).await?;
        // let msg = ws.receive().await?;
        // println!("msg: {:?}", msg);
        let (ws_stream, _) = connect_async(url).await.expect("failed to connect");
        let (write, read) = ws_stream.split();

        let client_to_ws = send_rx.map(Ok).forward(write);
        let ws_to_client = {
            read.for_each(|message| async {
                match message {
                    Ok(message) => match recv_tx.unbounded_send(message) {
                        Ok(_) => {}
                        Err(e) => {
                            error!("recv_tx err: {}", e)
                        }
                    },
                    Err(e) => {
                        error!("err on connection: {}", e)
                    }
                };
            })
        };

        // tokio::spawn(client_to_ws);
        // tokio::spawn(ws_to_client);
        pin_mut!(client_to_ws, ws_to_client);
        // future::select(client_to_ws, ws_to_client);
        future::select(client_to_ws, ws_to_client).await;

        // {
        //     // let s = s.clone();
        //     tokio::spawn(async move {
        //         loop {
        //             match send_rx.try_recv() {
        //                 Ok(v) => match v {
        //                     Ok(frame) => {
        //                         match ws.send(frame).await {
        //                             Ok(_) => {}
        //                             Err(e) => {
        //                                 error!("ws error: {}", e)
        //                             }
        //                         };
        //                     }
        //                     Err(e) => {
        //                         error!("bad frame: {}", e)
        //                     }
        //                 },
        //                 Err(e) => {
        //                     error!("recv send_tx err: {}", e)
        //                 }
        //             }
        //         }
        //     });
        // }
        // ws.close(None).await?;

        Ok(s)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod test {
    use crate::client::ChatinoClient;
    use anyhow::Result;

    #[test]
    fn test_client() -> Result<()> {
        let runner = async {
            let _ = ChatinoClient::new().await.unwrap();
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(runner);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl ChatinoClient {}
