use std::time::Duration;

use simple_async::{async_main, futures, task::Task};

async_main! {
    foo().await;
}

async fn foo() {
    println!("Spawning Task A");
    Task::spawn(async {
        println!("Task A started");

        futures::sleep(Duration::from_millis(1000)).await;

        println!("Task A finished");
    });

    println!("Spawning Task B");
    Task::spawn(async {
        println!("Task B started");

        futures::sleep(Duration::from_millis(2500)).await;

        println!("Task B finished");
    });

    println!("Main finished");
}
