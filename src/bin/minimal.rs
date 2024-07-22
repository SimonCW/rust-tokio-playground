use std::time::Instant;
use tokio::time::Duration;

async fn sleep_then_print(timer: i32) {
    println!("Start timer {}.", timer);

    tokio::time::sleep(Duration::from_secs(1)).await;
    //                                            ^ execution can be paused here

    println!("Timer {} done.", timer);
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // The join! macro lets you run multiple things concurrently.
    tokio::join!(
        sleep_then_print(1),
        sleep_then_print(2),
        sleep_then_print(3),
    );
    /*
    sleep_then_print(1).await;
    sleep_then_print(2).await;
    sleep_then_print(3).await;
    */
    println!("{:.2}", start.elapsed().as_secs_f64());
}
