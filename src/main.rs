use log::info;
use log4rs::init_file;


use crate::http::get_company_list;
use crate::json::Query;

mod json;
mod cache;
mod http;


#[tokio::main]
async fn main() -> Result<(), String> {
    init_file("config/log4rs.yaml", Default::default()).unwrap();

    let _ = get_company_list(&Query::default()).await;

    info!("Hello, world!");

    Ok(())
}
