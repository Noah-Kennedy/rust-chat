use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use postgres_types::{ToSql, FromSql};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, ToSql, FromSql)]
pub struct User {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, ToSql, FromSql)]
pub struct Msg {
    pub to: String,
    pub from: String,
    pub body: String,
    pub time: DateTime<Utc>,
}