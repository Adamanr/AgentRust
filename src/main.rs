extern crate core;

mod assets;
mod common;

use std::thread::sleep;
use std::time::Duration;
use crate::assets::tcp::tcp_client::tcp_c::start_client;
use crate::common::config::config::config::{Config, get_config};

#[tokio::main]
async fn main() {
    println!("---- PREPARE ----");
    sleep(Duration::from_secs(1));
    println!("---- START ----");
    let f = get_config().await;
    start_client(f).await;

}