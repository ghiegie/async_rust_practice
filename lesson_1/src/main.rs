use futures::executor::block_on;

use lesson_1::*;

fn main() {
    // let future = test();
    // block_on(future);

    // let future = test_await();
    // block_on(future);

    let future = test_futures_join();
    block_on(future);
}
