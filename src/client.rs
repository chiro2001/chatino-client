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
    pub async fn new(
        // send_tx: &UnboundedSender<Message>,
        send_rx: UnboundedReceiver<Message>,
        // recv_tx: UnboundedSender<Message>,
        // recv_rx: &UnboundedReceiver<Message>,
    ) -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        // let (send_tx, send_rx) = futures_channel::mpsc::unbounded();
        // let (recv_tx, recv_rx) = futures_channel::mpsc::unbounded();
        // ws.send_text("{\"cmd\":\"getinfo\"}".to_string()).await?;
        let (ws_stream, _) = connect_async(url).await.expect("failed to connect");
        let (write, read) = ws_stream.split();

        let client_to_ws = send_rx.map(Ok).forward(write);
        // let arc_recv_tx = Arc::new(recv_tx);
        // let ws_to_client = {
        //     // let recv_tx = arc_recv_tx.clone();
        //     read.for_each(|message| async {
        //         match message {
        //             Ok(message) => match recv_tx.unbounded_send(message) {
        //                 Ok(_) => {}
        //                 Err(e) => {
        //                     error!("recv_tx err: {}", e)
        //                 }
        //             },
        //             Err(e) => {
        //                 error!("err on connection: {}", e)
        //             }
        //         };
        //     })
        // };
        // let f = || {
        //     // a.await;
        //     tokio::spawn(client_to_ws);
        //     tokio::spawn(ws_to_client);
        //     ()
        // };
        // tokio::spawn(future::select(client_to_ws, ws_to_client));
        tokio::spawn(client_to_ws);
        // tokio::spawn(ws_to_client);
        Ok(Self {
            protocol,
            server,
            port,
            // send_tx,
            // send_rx,
            // recv_rx,
            // recv_tx,
            // runner: Box::new(f),
            reader: read,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod test {
    use crate::client::ChatinoClient;
    use anyhow::Result;
    use futures_util::StreamExt;
    use log::error;

    #[test]
    fn test_client() -> Result<()> {
        let (send_tx, send_rx) = futures_channel::mpsc::unbounded();
        let runner = async {
            let r = ChatinoClient::new(send_rx).await.unwrap();
            let read = r.reader;
            let ws_to_client = {
                // let recv_tx = arc_recv_tx.clone();
                read.for_each(|message| async {
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
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(runner);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl ChatinoClient {}
