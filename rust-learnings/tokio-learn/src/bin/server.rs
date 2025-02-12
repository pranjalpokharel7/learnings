use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type SharedDB = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (s, _) = listener.accept().await.unwrap();

        // process(s).await; // this is NOT concurrent as it processes one request at a time

        let db = db.clone();

        // tokio task - this spawns an asynchronous 'green thread' i.e. it is not a native OS thread
        // lifetime of spawned task must be static i.e. it must not contain references to data owned outsied the task
        tokio::spawn(async move {
            process(s, db).await;
        });
    }
}

// the process function actually moves the socket so we can omit the move in the spawn
async fn process(s: TcpStream, db: SharedDB) {
    // new connection
    let mut conn = Connection::new(s);

    if let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                println!("GET: {:?}", cmd.key());
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            Set(cmd) => {
                let mut db_guard = db.lock().unwrap();
                db_guard.insert(cmd.key().to_string(), cmd.value().to_vec().into());
                println!("SET: {:?}", cmd.key());
                Frame::Simple("OK".to_string())
            },
            cmd => Frame::Error(format!("unimplemented: {:?}", cmd)),
        };

        conn.write_frame(&response).await.unwrap();
    }
}
