pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use log::{error, info};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
    pub reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new(sender: UnboundedReceiver<Message>) -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| {
                error!("failed to connect: {}", e);
                e
            })
            .unwrap();
        let (write, reader) = ws_stream.split();
        let client_to_ws = sender
            .map(|x| {
                info!("client_to_ws sender: {}", x);
                x
            })
            .map(Ok)
            .forward(write);
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
    use crate::client::{ChatinoClient, PORT, PROTOCOL, SERVER};
    use anyhow::Result;
    use futures_util::{future, pin_mut, StreamExt};
    use log::{error, info, warn};
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::time::sleep;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message;

    async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
        let mut stdin = tokio::io::stdin();
        loop {
            let mut buf = vec![0; 1024];
            let n = match stdin.read(&mut buf).await {
                Err(_) | Ok(0) => break,
                Ok(n) => n,
            };
            buf.truncate(n);
            tx.unbounded_send(Message::binary(buf)).unwrap();
        }
    }

    #[test]
    fn test_websocket() -> Result<()> {
        let runner = async {
            // let url = format!("{}://{}:{}/", PROTOCOL, SERVER, PORT);
            let url = format!("{}://{}:{}/", "ws", "localhost", 12345);
            let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
            tokio::spawn(read_stdin(stdin_tx));

            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            println!("WebSocket handshake has been successfully completed");

            let (write, read) = ws_stream.split();

            let stdin_to_ws = stdin_rx.map(Ok).forward(write);
            let ws_to_stdout = {
                read.for_each(|message| async {
                    let data = message.unwrap().into_data();
                    tokio::io::stdout().write_all(&data).await.unwrap();
                })
            };

            pin_mut!(stdin_to_ws, ws_to_stdout);
            future::select(stdin_to_ws, ws_to_stdout).await;
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(runner);
        warn!("progress exited");
        Ok(())
    }

    #[test]
    fn test_client() -> Result<()> {
        tracing_subscriber::fmt::init();
        let (sender, send_rx) = futures_channel::mpsc::unbounded();
        let runner = async {
            let r = ChatinoClient::new(send_rx).await.unwrap();
            let reader = r.reader;
            let ws_to_client = async {
                sleep(Duration::from_millis(1700)).await;
                reader
                    .for_each(|message| async {
                        match message {
                            Ok(message) => {
                                println!("recv message: {}", message)
                            }
                            Err(e) => {
                                error!("err on connection: {}", e)
                            }
                        };
                    })
                    .await;
                warn!("reader to stdout exited");
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
