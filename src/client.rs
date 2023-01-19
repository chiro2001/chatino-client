// pub const WSS_URL: &'static str = "wss://ws.crosst.chat:35197/";
pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
use log::error;
use tokio::sync::mpsc::{channel, Sender};
use websockets::Frame;

#[derive(Debug, Clone)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
    pub send_tx: Sender<Result<Frame, &'static str>>,
    pub recv_tx: Sender<Result<Frame, &'static str>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new() -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        let mut ws = websockets::WebSocket::connect(&url).await?;
        let (send_tx, mut send_rx) = channel(1024);
        let (recv_tx, recv_rx) = channel(1024);
        let s = Self {
            protocol,
            server,
            port,
            send_tx,
            recv_tx,
        };
        ws.send_text("{\"cmd\":\"getinfo\"}".to_string()).await?;
        let msg = ws.receive().await?;
        println!("msg: {:?}", msg);

        let delay = 50;
        {
            // let s = s.clone();
            tokio::spawn(async move {
                loop {
                    match send_rx.try_recv() {
                        Ok(v) => match v {
                            Ok(frame) => {
                                match ws.send(frame).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error!("ws error: {}", e)
                                    }
                                };
                            }
                            Err(e) => {
                                error!("bad frame: {}", e)
                            }
                        },
                        Err(e) => {
                            error!("recv send_tx err: {}", e)
                        }
                    }
                }
            });
        }
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
