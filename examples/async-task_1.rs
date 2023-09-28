
async fn async_fn_1() {
    println!("from async_fn_1");
}

#[tokio::main]
async fn main() {
    // 一个设置成future的表示
    let future_async_fn = async_fn_1();

    println!("from main");

    // 这个时候future才被执行
    future_async_fn.await;
}