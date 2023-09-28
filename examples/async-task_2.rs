use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(async_task(i)));
    }

    println!("[thread main] start task main");
    std::thread::sleep(Duration::from_millis(750));
    println!("[thread main] finish task main");

    for handle in handles {
        // `spawn` 方法返回一个 `JoinHandle`，它是一个 `Future`，因此可以通过  `block_on` 来等待它完成
        runtime.block_on(handle).unwrap();
    }
}

async fn async_task(i: u64) {
    println!("[tokio] start task {}",i);

    let millis = 1000 - 50 * i;
    sleep(Duration::from_millis(millis)).await;

    println!("[tokio] finish Task {}",i);
}