use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame,Command::{self, Get, Set}};
use std::collections::HashMap;
use bytes::Bytes;
use std::sync::Arc;


/* 1. 使用 Tokio 提供的锁: 锁如果在多个.await的过程中持有，锁可能在线程间转移，某个任务刚获取完锁，还没使用完就因为 .await 让出了当前线程的所有权，结果下个任务又去获取了锁，造成死锁
   2. 使用 std   提供的锁: 只在单个任务.await持有，中间不经历其他异步任务 的情况下使用 */
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex as TokioMutex;


type DataBase = Arc<StdMutex<HashMap<String, Bytes>>>;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let database = Arc::new(StdMutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let database = database.clone();

        /* move会把克隆好的共享数据指针所有权传进子任务中 使得每个任务都可以获取到共享数据
        这里就是一个经典的创建异步任务的流程  */
        tokio::spawn(async move {
            process_request(socket, database).await;
        });
    }
}


async fn process_request(socket: TcpStream, database: DataBase) {

    let mut connection = Connection::new(socket);

    // 这里的代码就是使用mini-redis的库函数来处理我们收到的命令
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = database.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = database.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}