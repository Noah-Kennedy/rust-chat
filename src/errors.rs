use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Pg(tokio_postgres::Error),
}