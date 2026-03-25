mod domain;

use domain::cart::Cart;
use axum::{Json, Router, routing::get};
use serde_json::json;
use std::time::Instant;



async fn hello() -> Json<serde_json::Value> {
    Json(json!({
        "message": "HelloWorld"
    }))
}

#[tokio::main]
async fn main() {
    // ⏱️ start pomiaru
    let start = Instant::now();

    // domain code

    let mut cart = Cart::new();

    cart.add("product-1".to_string(), 2);
    cart.add("product-1".to_string(), 3);
    cart.add("product-2".to_string(), 1);

    println!("Cart after add: {:?}", cart);

    cart.delete("product-1");

    println!("Cart after delete: {:?}", cart);

    //end of domain code

    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // ⏱️ koniec "bootowania"
    let duration = start.elapsed();
    println!("🚀 Serwer wystartował w: {:?}", duration);
    println!("🌍 http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
