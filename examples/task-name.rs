//! Spawns a named task that prints its name.

use async_std::thread;

async fn print_name() {
    println!("My name is {:?}", task::current().name());
}

fn main() {
    thread::spawn_task(async {
        task::Builder::new()
            .name("my-task".to_string())
            .spawn(print_name())
            .unwrap()
            .await;
    })
}
