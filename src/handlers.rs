use actix_web::{web, HttpResponse, HttpRequest, Error};
use crate::model::{User, Msg};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use bb8_postgres::bb8::Pool;
use crate::util::{handle_insert_results, make_db_pool};
use actix_web_actors::ws;
use crate::ws::WebsocketActor;

/// Application state
/// Thread safe
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<PostgresConnectionManager<NoTls>>
}

impl AppState {
    pub async fn new() -> Self {
        let pool = make_db_pool().await;

        Self { pool }
    }
}

/// POST /users JSON{User}
pub async fn add_user(state: web::Data<AppState>, user: web::Json<User>) -> HttpResponse {
    debug!("Adding user {}", user.username);

    let db = state.pool.get().await.unwrap();

    let res = db.query(
        "INSERT INTO users (username) VALUES ($1) RETURNING *",
        &[&user.username])
        .await;

    handle_insert_results(res)
}

/// POST /messages JSON{Msg}
pub async fn send_message(state: web::Data<AppState>, msg: web::Json<Msg>) -> HttpResponse {
    debug!("Sending message {:?}", msg.0);

    let db = state.pool.get().await.unwrap();

    let res = db.query(
        "INSERT INTO messages (\"to\", \"from\", \"body\", \"time\") VALUES ($1, $2, $3, $4) RETURNING *",
        &[&msg.to, &msg.from, &msg.body, &msg.time])
        .await;

    handle_insert_results(res)
}

/// GET /messages JSON{User}
pub async fn get_messages(state: web::Data<AppState>, user: web::Json<User>) -> HttpResponse {
    debug!("Getting messages for user {}", user.username);
    HttpResponse::NotImplemented().finish()
}

/// GET /users
pub async fn get_users(state: web::Data<AppState>) -> HttpResponse {
    debug!("Getting all users");
    HttpResponse::NotImplemented().finish()
}

/// GET /ws
pub async fn get_websocket(
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
)
    -> Result<HttpResponse, Error>
{
    let websocket = WebsocketActor {
        app_state: state
    };

    ws::start(websocket, &req, stream)
}