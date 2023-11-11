pub async fn test() {
    println!("test");
}

pub async fn test1() {
    println!("test 1");
}

pub async fn test2() {
    println!("test 2");
}

pub async fn test3() {
    println!("test 3");
}

pub async fn test_await() {
    let test1 = test1().await;
    let test2 = test2().await;
}

pub async fn test_futures_join() {
    let f1 = test1();
    let f2 = test2();

    futures::join!(f1, f2);
}