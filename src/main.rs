use std::{
    thread::sleep,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() {
    println!("Start ...");
    let start = Instant::now();

    // Spawns task in background thread
    let io_task = tokio::spawn(async_io_func());

    let syn_res = sync_func();
    println!("{:?}: {}", start.elapsed(), syn_res);

    // will print shortly after sync because much works has already be done in the background
    let io_res = io_task.await.unwrap();
    println!("{:?}: {}", start.elapsed(), io_res);
    println!("{:?}: I'm last 🦀", start.elapsed());
}

fn sync_func() -> String {
    sleep(Duration::from_millis(500));
    return String::from("Sync Result");
}

async fn async_io_func() -> String {
    sleep(Duration::from_millis(500));
    String::from("Async Result")
}
