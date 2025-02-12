use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    // capacity = 2, means 2 messages can be exchanged at once before buffering
    let (tx, mut rx) = mpsc::channel(2);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx.send(Command::Set {
            key: "hello".to_owned(),
            val: "world".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let resp = resp_rx.await;
        println!("{:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx2.send(Command::Get {
            key: "hello".to_owned(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let resp = resp_rx.await;
        println!("{:?}", resp);
    });

    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            // need to re-establish connection with server because it is not persistent
            let mut client = client::connect("127.0.0.1:6379").await.unwrap();

            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            };
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
