#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use reqwest;
use rocket::response::content;
use std::time::Duration;

static mut PRICE: String = String::new();

#[get("/")]
fn json() -> content::Json<&'static str> {
    unsafe {
        content::Json(PRICE.as_str())
    }
}

#[tokio::main]
async fn get() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.coingecko.com/api/v3/coins/solana").send().await?;
    let body = res.text().await?;
    {
        unsafe {
            PRICE = body;
        }
    }
    Ok(())
}

fn init_planner() {
    let mut planner = periodic::Planner::new();
    planner.add(
        || drop(get()),
        periodic::Every::new(Duration::from_secs(10)),
    );
    planner.start();
}

fn main() {
    drop(get());
    init_planner();
    rocket::ignite().mount("/", routes![json]).launch();
}