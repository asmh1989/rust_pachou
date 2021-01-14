#![allow(dead_code)]

use log::info;

use crate::http::get_company_list;
use crate::json::Query;

mod cache;
mod http;
mod json;

#[tokio::main]
async fn main() -> Result<(), String> {
    let _ = get_company_list(&Query::default()).await;

    info!("Hello, world!");

    Ok(())
}
