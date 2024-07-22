use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
struct UserData {
    id: u32,
    name: String,
}

#[derive(Serialize)]
struct OrderHistory {
    id: u32,
    total: f64,
}

#[derive(Serialize)]
struct UserInfo {
    user: UserData,
    orders: Vec<OrderHistory>,
}

async fn fetch_user_data(user_id: u32) -> UserData {
    // Simulate a time-consuming database query
    sleep(Duration::from_millis(500)).await;
    UserData {
        id: user_id,
        name: format!("User {}", user_id),
    }
}

async fn fetch_order_history(user_id: u32) -> Vec<OrderHistory> {
    // Simulate a time-consuming database query
    sleep(Duration::from_millis(500)).await;
    vec![
        OrderHistory {
            id: 1,
            total: 99.99,
        },
        OrderHistory {
            id: 2,
            total: 149.99,
        },
    ]
}

async fn get_user_info(Path(user_id): Path<u32>) -> Json<UserInfo> {
    // Sequential approach (within request)
    let user = fetch_user_data(user_id).await;
    let orders = fetch_order_history(user_id).await;

    Json(UserInfo { user, orders })
}

async fn get_user_info_optimized(Path(user_id): Path<u32>) -> Json<UserInfo> {
    // Within request parallel approach using tokio::join!
    let (user, orders) = tokio::join!(fetch_user_data(user_id), fetch_order_history(user_id));

    Json(UserInfo { user, orders })
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/user/:id", get(get_user_info))
        .route("/user_optimized/:id", get(get_user_info_optimized));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
