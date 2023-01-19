pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use log::{error, info, warn};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
    pub reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new() -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        // let url = format!("{}://{}:{}/", "ws", "localhost", 12345);
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| {
                error!("failed to connect: {}", e);
                e
            })
            .unwrap();
        let (writer, reader) = ws_stream.split();
        Ok(Self {
            protocol,
            server,
            port,
            reader,
            writer,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod test {
    use crate::client::{ChatinoClient, PORT, PROTOCOL, SERVER};
    use anyhow::Result;
    use futures_util::{future, pin_mut, StreamExt};
    use log::{error, info, warn};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::Mutex;
    use tokio::time::sleep;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message;

    async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
        loop {
            // let mut stdin = tokio::io::stdin();
            // let mut buf = vec![0; 1024];
            // let n = match stdin.read(&mut buf).await {
            //     Err(_) | Ok(0) => break,
            //     Ok(n) => n,
            // };
            // buf.truncate(n);
            // tx.unbounded_send(Message::binary(buf)).unwrap();
            let v = "{\"cmd\": \"getinfo\"}".to_string();
            info!("sent: {}", v);
            tx.unbounded_send(Message::Text(v)).unwrap();
            sleep(Duration::from_millis(1000)).await;
            // tx.unbounded_send(Message::binary("{\"cmd\": \"getinfo\"}".to_string())).unwrap();
        }
        warn!("read_stdin finished");
    }

    #[test]
    fn test_websocket() -> Result<()> {
        tracing_subscriber::fmt::init();
        let stop = Arc::new(Mutex::new(false));
        let stop_in_runner = stop.clone();
        let runner = async move {
            let url = format!("{}://{}:{}/", PROTOCOL, SERVER, PORT);
            // let url = format!("{}://{}:{}/", "ws", "localhost", 12345);
            let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
            // tokio::spawn(read_stdin(stdin_tx));
            let stop = stop_in_runner.clone();
            tokio::spawn(async move {
                let mut i = 0;
                loop {
                    let v = "{\"cmd\": \"getinfo\"}".to_string();
                    info!("sent: {}", v);
                    stdin_tx.unbounded_send(Message::Text(v)).unwrap();
                    sleep(Duration::from_millis(1000)).await;
                    if i > 3 {
                        break;
                    }
                    i += 1;
                }
                let mut s = stop.lock().await;
                *s = true;
            });

            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            println!("WebSocket handshake has been successfully completed");

            let (write, read) = ws_stream.split();

            let stdin_to_ws = stdin_rx.map(Ok).forward(write);
            let ws_to_stdout = {
                read.for_each(|message| async {
                    // let data = message.unwrap().into_data();
                    // tokio::io::stdout().write_all(&data).await.unwrap();
                    info!("ws_to_stdout: message = {}", message.unwrap());
                })
            };

            pin_mut!(stdin_to_ws, ws_to_stdout);
            // future::select(stdin_to_ws, ws_to_stdout).await;
            future::join(stdin_to_ws, ws_to_stdout).await;
            while !*stop_in_runner.lock().await {
                sleep(Duration::from_millis(100)).await;
            }
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(runner);
        warn!("runner exited...");
        Ok(())
    }

    #[test]
    fn test_client() -> Result<()> {
        tracing_subscriber::fmt::init();
        let (send_tx, send_rx) = futures_channel::mpsc::unbounded();
        let runner = async {
            let r = ChatinoClient::new().await.unwrap();

            let send_async = async move {
                loop {
                    sleep(Duration::from_millis(1000)).await;
                    match send_tx.unbounded_send(Message::Text("{\"cmd\":\"getinfo\"}".to_string()))
                    {
                        Ok(_) => {
                            info!("message sent");
                        }
                        Err(e) => {
                            error!("sender error: {}", e);
                            panic!("{}", e);
                        }
                    }
                }
            };
            tokio::spawn(send_async);
            // tokio::spawn(read_stdin(sender));
            let client_to_ws = send_rx
                .map(|x| {
                    info!("client_to_ws: {}", x);
                    x
                })
                .map(Ok)
                .forward(r.writer);
            let ws_to_stdout = {
                let r = r.reader.for_each(|message| async {
                    let data = message.unwrap().into_data();
                    tokio::io::stdout().write_all(&data).await.unwrap();
                });
                warn!("ws_to_stdout start? finished");
                r
            };
            pin_mut!(client_to_ws, ws_to_stdout);
            // ws_to_stdout.await;
            future::select(ws_to_stdout, client_to_ws).await;
            // future::join(ws_to_stdout, client_to_ws).await;
            warn!("sleeping...");
            sleep(Duration::from_millis(10000)).await;
            warn!("all finished");
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(runner);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl ChatinoClient {}
