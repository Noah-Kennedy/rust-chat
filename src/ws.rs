use actix_web::web;
use crate::handlers::AppState;
use actix::{Actor, StreamHandler, ActorContext, spawn};
use actix_web_actors::ws;
use actix_web_actors::ws::{ProtocolError, Message};
use crate::model::Msg;

/// Custom actor model for websocket
/// Handles socket events
pub struct WebsocketActor {
    pub app_state: web::Data<AppState>,
}

impl Actor for WebsocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for WebsocketActor {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(Message::Pong(_)) => {
                unimplemented!()
            }
            Ok(Message::Text(text)) => {
                match serde_json::from_str(&text) {
                    Ok(msg) => {
                        let msg: Msg = msg;
                        let state = self.app_state.clone();

                        let fut = async move {
                            let db = state.pool.get().await.unwrap();

                            let res = db.query(
                                "INSERT INTO messages (\"to\", \"from\", \"body\", \"time\") VALUES ($1, $2, $3, $4) RETURNING *",
                                &[&msg.to, &msg.from, &msg.body, &msg.time])
                                .await;
                        };

                        spawn(fut);
                    }
                    Err(e) => {
                        warn!("{}", e);
                    }
                }
            }
            Ok(Message::Binary(_)) => {
                unimplemented!()
            }
            Ok(Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}