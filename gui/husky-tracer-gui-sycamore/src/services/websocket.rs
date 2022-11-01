use std::{cell::Cell, collections::HashMap};

use crate::*;
use futures::{
    channel::mpsc::{Receiver, Sender},
    stream::SplitStream,
    SinkExt, StreamExt,
};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct WebsocketService {
    pub gui_message_sender: Sender<String>,
    // pub(super) call_backs: Rc<RefCell<HashMap<usize, Box<dyn FnOnce(HuskyTracerServerMessage)>>>>,
    next_request_id: Cell<usize>,
}

impl std::fmt::Debug for WebsocketService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebsocketService")
            .field("tx", &self.gui_message_sender)
            .finish()
    }
}

impl WebsocketService {
    pub fn new() -> (Self, Receiver<HuskyTracerServerMessage>) {
        let ws = WebSocket::open("ws://localhost:51617/query").unwrap();

        let (mut write, mut read) = ws.split();

        let (mut gui_message_sender, mut gui_message_receiver) =
            futures::channel::mpsc::channel::<String>(1000);

        let (mut server_notification_sender, mut server_notification_receiver) =
            futures::channel::mpsc::channel::<HuskyTracerServerMessage>(100);

        spawn_local(async move {
            while let Some(s) = gui_message_receiver.next().await {
                write.send(Message::Text(s)).await.unwrap();
            }
        });
        let this = Self {
            gui_message_sender,
            next_request_id: Cell::new(0),
            // call_backs: Rc::new(RefCell::new(HashMap::new())),
        };
        spawn_local({
            let this = this.clone();
            async move {
                while let Some(msg) = read.next().await {
                    let server_message: HuskyTracerServerMessage = match msg {
                        Ok(Message::Text(ref data)) => match serde_json::from_str(data) {
                            Ok(msg) => msg,
                            Err(e) => {
                                log::info!("{}", data);
                                panic!("{}", e)
                            }
                        },
                        Ok(Message::Bytes(b)) => {
                            let decoded = std::str::from_utf8(&b);
                            if let Ok(data) = decoded {
                                serde_json::from_str(data).unwrap()
                            } else {
                                log::error!("what");
                                continue;
                            }
                        }
                        Err(e) => {
                            log::error!("what");
                            continue;
                        }
                    };
                    todo!("process change");
                    if let Some(request_id) = server_message.opt_request_id {
                        todo!()
                        // this.call_backs
                        //     .borrow_mut(file!(), line!())
                        //     .remove(&request_id)
                        //     .unwrap()(server_message);
                    } else {
                        server_notification_sender
                            .send(server_message)
                            .await
                            .unwrap()
                    }
                }
                log::debug!("WebSocket Closed");
            }
        });
        (this, server_notification_receiver)
    }

    pub fn issue_request_id(&self) -> usize {
        let request_id = self.next_request_id.get();
        self.next_request_id.set(request_id + 1);
        request_id
    }

    pub fn send_message(&self, variant: HuskyTracerGuiMessageVariant, needs_response: bool) {
        let request = HuskyTracerGuiMessage {
            opt_request_id: if needs_response {
                Some(self.issue_request_id())
            } else {
                None
            },
            variant,
        };
        spawn_local({
            let mut gui_message_sender = self.gui_message_sender.clone();
            async move {
                gui_message_sender
                    .send(serde_json::to_string(&request).unwrap())
                    .await;
            }
        });
    }
}
