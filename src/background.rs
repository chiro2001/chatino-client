use crate::chatino::Action;
use crate::client::{ChatinoClient, CLIENT_KEY, CLIENT_NAME};
use crate::message::{
    get_chat_type, get_cmd, CmdChatNormal, CmdChatWhisper, CmdGetInfoReq, CmdInfo, CmdJoinReq,
    CmdOnlineAdd, CmdOnlineRemove, CmdOnlineSet,
};
use anyhow::Result;
use futures_util::{future, pin_mut, StreamExt};
use log::{debug, info};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

pub async fn background(tx: Sender<Action>, rx: Receiver<Action>) -> Result<()> {
    let tx = Arc::new(Mutex::new(tx));
    info!("background task launching");
    debug!("connecting to server");
    let client = ChatinoClient::new().await?;
    let client_available = Arc::new(Mutex::new(true));
    // let stop = Arc::new(Mutex::new(false));
    let (ws_send_tx, ws_send_rx) = futures_channel::mpsc::unbounded();
    // let stop_send = stop.clone();
    let client_to_ws = ws_send_rx.map(Ok).forward(client.writer);
    let ws_to_ui = {
        client.reader.for_each(|message| async {
            if let Ok(message) = message {
                if *client_available.lock().await {
                    match message {
                        Message::Text(text) => match get_cmd(&text) {
                            None => {}
                            Some(cmd) => {
                                info!("[{}] {}", cmd, text);
                                match cmd.as_str() {
                                    "onlineSet" => {
                                        let v: CmdOnlineSet = serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::OnlineSet(v)).unwrap();
                                    }
                                    "onlineRemove" => {
                                        let v: CmdOnlineRemove =
                                            serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::OnlineRemove(v)).unwrap();
                                    }
                                    "onlineAdd" => {
                                        let v: CmdOnlineAdd = serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::OnlineAdd(v)).unwrap();
                                    }
                                    "info" => {
                                        let v: CmdInfo = serde_json::from_str(&text).unwrap();
                                        tx.lock().await.send(Action::Info(v)).unwrap();
                                    }
                                    "chat" => match get_chat_type(&text) {
                                        None => {}
                                        Some(type_name) => match type_name.as_str() {
                                            "chat" => {
                                                let v: CmdChatNormal =
                                                    serde_json::from_str(&text).unwrap();
                                                tx.lock()
                                                    .await
                                                    .send(Action::ChatNormal(v))
                                                    .unwrap();
                                            }
                                            "whisper" => {
                                                let v: CmdChatWhisper =
                                                    serde_json::from_str(&text).unwrap();
                                                tx.lock()
                                                    .await
                                                    .send(Action::ChatWhisper(v))
                                                    .unwrap();
                                            }
                                            _ => {}
                                        },
                                    },
                                    _ => {
                                        tx.lock()
                                            .await
                                            .send(Action::RaiseError(format!(
                                                "Unimplemented command: {}",
                                                cmd
                                            )))
                                            .unwrap();
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        })
    };
    pin_mut!(client_to_ws, ws_to_ui);
    let task = async move {
        loop {
            // let client_available = client_available.clone();
            match rx.try_recv() {
                Ok(action) => match action {
                    Action::GetInfo => {
                        ws_send_tx
                            .unbounded_send(Message::Text(
                                serde_json::to_string(&CmdGetInfoReq::default()).unwrap(),
                            ))
                            .unwrap();
                        // *client_available.lock().await = false;
                    }
                    Action::Login(channel, nick, password) => {
                        ws_send_tx
                            .unbounded_send(Message::Text(
                                serde_json::to_string(&CmdJoinReq {
                                    cmd: "join".to_string(),
                                    channel,
                                    nick,
                                    password,
                                    clientName: CLIENT_NAME.to_string(),
                                    clientKey: CLIENT_KEY.to_string(),
                                })
                                .unwrap(),
                            ))
                            .unwrap();
                    }
                    Action::SendMessage(_) => {}
                    _ => {}
                },
                Err(_) => {}
            }
            sleep(Duration::from_millis(10)).await;
        }
    };
    tokio::spawn(task);
    debug!("before select...");
    future::select(ws_to_ui, client_to_ws).await;
    info!("background task launched");
    // loop {
    //     // let client_available = client_available.clone();
    //     match rx.try_recv() {
    //         Ok(action) => match action {
    //             Action::GetInfo => {
    //                 ws_send_tx
    //                     .unbounded_send(Message::Text(
    //                         serde_json::to_string(&CmdGetInfoReq::default()).unwrap(),
    //                     ))
    //                     .unwrap();
    //                 // *client_available.lock().await = false;
    //             }
    //             Action::Login(channel, nick, password) => {
    //                 ws_send_tx
    //                     .unbounded_send(Message::Text(
    //                         serde_json::to_string(&CmdJoinReq {
    //                             cmd: "join".to_string(),
    //                             channel,
    //                             nick,
    //                             password,
    //                             clientName: CLIENT_NAME.to_string(),
    //                             clientKey: CLIENT_KEY.to_string(),
    //                         })
    //                         .unwrap(),
    //                     ))
    //                     .unwrap();
    //             }
    //             Action::SendMessage(_) => {}
    //             _ => {}
    //         },
    //         Err(_) => {}
    //     }
    //     sleep(Duration::from_millis(10)).await;
    // }
    Ok(())
}
