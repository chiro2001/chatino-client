// pub const WSS_URL: &'static str = "wss://ws.crosst.chat:35197/";
pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;

use anyhow::Result;
// use std::sync::mpsc;
// use std::thread;
// use websocket::futures::{Future, Sink, Stream};
// use websocket::header::ProtocolName::WebSocket;
// use websocket::{ClientBuilder, OwnedMessage, WebSocketError};

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub async fn new() -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        // let mut builder = ClientBuilder::new(&url)?
        //     // unused
        //     .add_protocol("chatino");
        // let (send, recv) = mpsc::channel();
        // if protocol == "wss" {
        //     // let mut client = builder.connect_secure(None)?;
        //     // client.send_message(ws::Message::)?;
        //     // for message in client.incoming_messages() {
        //     //     println!("message: {:?}", message);
        //     // }
        //     // let runner = builder.async_connect_secure(None)
        //     //     .and_then(|(dyplex, _)| {});
        //     // let mut runtime = tokio::runtime::current_thread::Builder::new()
        //     //     .build()
        //     //     .unwrap();
        //     // let _ = runtime.block_on(runner)?;
        // } else {
        //     let client = builder.connect_insecure()?;
        //     panic!("not tested");
        // };
        // thread::spawn(move || {
        //     send.send(OwnedMessage::Text("{\"cmd\":\"getinfo\"}".to_string())).unwrap();
        // });
        // let runner = ClientBuilder::new(&url)?
        //     .async_connect_secure(None)
        //     .and_then(|(duplex, _)| {
        //         let (sink, stream) = duplex.split();
        //         stream
        //             .filter_map(|message| {
        //                 println!("recv message: {:?}", message);
        //                 match message {
        //                     OwnedMessage::Text(text) => {
        //                         println!("text: {}", text);
        //                         None
        //                     }
        //                     _ => None,
        //                 }
        //             })
        //             // .select(recv.recv().map_err(|_| WebSocketError::NoDataAvailable))
        //             .forward(sink)
        //     });
        // let mut rt = tokio::runtime::Runtime::new()?;
        // // rt.block_on(runner);
        let mut ws = websockets::WebSocket::connect(&url).await?;
        ws.send_text("{\"cmd\":\"getinfo\"}".to_string()).await?;
        let msg = ws.receive().await?;
        println!("msg: {:?}", msg);
        ws.close(None).await?;

        Ok(Self {
            protocol,
            server,
            port,
        })
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
