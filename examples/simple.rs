use std::time::Duration;

use simple_async::{async_main, futures::sleep};

async_main! {
    println!("Test123");

    sleep(Duration::from_millis(1000)).await;

    println!("Done");
}
