use std::time::Duration;

use simple_async::{executor::Executor, futures::sleep};

fn main() {
    let mut executor = Executor::new();

    executor.spawn(foo());
    executor.spawn(bar());

    executor.run();
}

async fn foo() {
    println!("foo() starting");

    sleep(Duration::from_millis(1000)).await;

    println!("foo() done");
}

async fn bar() {
    println!("bar() starting");

    sleep(Duration::from_millis(2500)).await;

    println!("bar() done");
}
