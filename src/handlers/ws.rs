use actix::StreamHandler;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use futures_channel::mpsc::UnboundedSender;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
/// WebSocket handler function
async fn ws_connect(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocketSession, &req, stream)
}

/// Define WebSocket session struct
struct WebSocketSession;

impl actix::Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
}

/// Define how the WebSocket handles messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(format!("Echo: {}", text));
        }
    }
}

/// Initializes WebSocket routes
pub fn init_ws_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(ws_connect)));
}

/// Type alias for the sender channel that sends WebSocket messages.
type Tx = UnboundedSender<Message>;

/// A shared map of user IDs to their WebSocket sender.
pub type UserSocketMap = Arc<Mutex<HashMap<String, Tx>>>;

/// Sends a JSON payload to a specific user **as a JSON string**.
pub async fn send_to_user(user_sockets: &UserSocketMap, user_id: &str, payload: Value) {
    let sockets = user_sockets.lock().unwrap();
    if let Some(tx) = sockets.get(user_id) {
        if let Ok(json_string) = serde_json::to_string(&payload) {
            if let Err(e) = tx.unbounded_send(Message::text(json_string)) {
                eprintln!("Error sending message to {}: {:?}", user_id, e);
            }
        }
    }
}

/// Sends a JSON payload to multiple users **as JSON strings**.
pub async fn send_to_users(user_sockets: &UserSocketMap, user_ids: &[String], payload: Value) {
    for user_id in user_ids {
        send_to_user(user_sockets, user_id, payload.clone()).await;
    }
}

/// Sends a JSON payload to all connected users **as JSON strings**.
pub async fn send_to_all(user_sockets: &UserSocketMap, payload: Value) {
    let sockets = user_sockets.lock().unwrap();
    for (_user_id, tx) in sockets.iter() {
        if let Ok(json_string) = serde_json::to_string(&payload) {
            if let Err(e) = tx.unbounded_send(Message::text(json_string)) {
                eprintln!("Error sending message to user: {:?}", e);
            }
        }
    }
}
