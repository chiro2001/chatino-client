// pub const WSS_URL: &'static str = "wss://ws.crosst.chat:35197/";
pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
use queues::Queue;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use websockets::Frame;

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
    // pub send: Arc<Mutex<Queue<Frame>>>,
    // pub recv: Arc<Mutex<Queue<Frame>>>,
    send_tx: Sender<Result<Frame, &'static str>>,
    send_rx: Receiver<Result<Frame, &'static str>>,
    recv_tx: Sender<Result<Frame, &'static str>>,
    recv_rx: Receiver<Result<Frame, &'static str>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new() -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        let mut ws = websockets::WebSocket::connect(&url).await?;
        // let send = Arc::new(Mutex::new(Queue::new()));
        // let recv = Arc::new(Mutex::new(Queue::new()));
        let (send_tx, send_rx) = channel(1024);
        let (recv_tx, recv_rx) = channel(1024);
        let mut s = Self {
            protocol,
            server,
            port,
            // send,
            // recv,
            send_rx,
            send_tx,
            recv_rx,
            recv_tx,
        };
        ws.send_text("{\"cmd\":\"getinfo\"}".to_string()).await?;
        let msg = ws.receive().await?;
        println!("msg: {:?}", msg);

        tokio::spawn(async move {
            loop {
                match s.send_rx.recv().await {
                    Some(v) => match v {
                        Ok(frame) => ws.send(frame),
                        Err(_) => {}
                    },
                    None => {}
                };
            }
        });
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
