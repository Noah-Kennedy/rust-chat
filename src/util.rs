use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Row};
use bb8_postgres::bb8::Pool;
use std::env;
use actix_web::HttpResponse;

/// Create pool of database connections
pub async fn make_db_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    let url = env::var("DATABASE_URL").unwrap();

    let manager = PostgresConnectionManager::new_from_stringlike(&url, NoTls).unwrap();

    Pool::builder()
        .max_size(32)
        .build(manager)
        .await
        .unwrap()
}

/// Handle results from a DB insertion operation.
pub fn handle_insert_results(res: Result<Vec<Row>, tokio_postgres::Error>) -> HttpResponse {
    match res {
        Ok(rows) => {
            if rows.len() == 1 {
                HttpResponse::Created().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(e) => {
            warn!("{}", e);
            HttpResponse::Conflict().finish()
        }
    }
}
