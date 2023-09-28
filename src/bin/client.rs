use mini_redis::client;
use Command::*;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio;


#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}


#[tokio::main]
async fn main() {
   /*  let mut client = client::connect("127.0.0.1:6379").await.unwrap();


    let (tx, mut rx) = mpsc::channel(32);
    let tx1 = tx.clone();
    let tx2 = tx.clone();


    // 生成两个任务，一个用于获取 key，一个用于设置 key
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string(),
        };
        tx1.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    // 生成一个任务管理任务
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key } => {
                    let _ = client.get(&key).await;
                }
                Set { key, val } => {
                    let _ = client.set(&key, val).await;
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap(); */
}