// pub const WSS_URL: &'static str = "wss://ws.crosst.chat:35197/";
pub const PROTOCOL: &'static str = "wss";
pub const SERVER: &'static str = "ws.crosst.chat";
pub const PORT: u16 = 35197;
use anyhow::Result;
use websocket::{ClientBuilder, ws};
use websocket::futures::Future;

#[derive(Debug)]
pub struct ChatinoClient {
    pub protocol: String,
    pub server: String,
    pub port: u16,
}

#[cfg(not(target_arch = "wasm32"))]
impl ChatinoClient {
    pub fn new() -> Result<Self> {
        let protocol = PROTOCOL.to_string();
        let server = SERVER.to_string();
        let port = PORT;
        let url = format!("{}://{}:{}/", protocol, server, port);
        let mut builder = ClientBuilder::new(&url)?
            // unused
            .add_protocol("chatino");
        if protocol == "wss" {
            // let mut client = builder.connect_secure(None)?;
            // client.send_message(ws::Message::)?;
            // for message in client.incoming_messages() {
            //     println!("message: {:?}", message);
            // }
            // let runner = builder.async_connect_secure(None)
            //     .and_then(|(dyplex, _)| {});
            // let mut runtime = tokio::runtime::current_thread::Builder::new()
            //     .build()
            //     .unwrap();
            // let _ = runtime.block_on(runner)?;
        } else {
            let _client = builder.connect_insecure()?;
            panic!("not tested");
        };

        Ok(Self {
            protocol,
            server,
            port,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod test {
    use anyhow::Result;
    use crate::client::ChatinoClient;

    #[test]
    fn test_client() -> Result<()> {
        let _ = ChatinoClient::new()?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl ChatinoClient {}
