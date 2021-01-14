#![allow(dead_code)]

use log::info;

use crate::json::{Company};
use crate::cache::{get_cache, save_cache_with_path, ResultOutput};
use std::env::args;

mod cache;
mod http;
mod json;

#[tokio::main]
async fn main() -> Result<(), String> {
    crate::cache::init_cache();

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        info!("请输入搜索的资质!!");
        return Ok(());
    }

    let filter = args.get(1).unwrap();
    let cache = get_cache();

    let mut data:Vec<Company> = Vec::new();

    let mut hui_list:Vec<String> = Vec::new();

    for c in cache.company_list  {
        if let Some(list) = c.qualification.clone() {
            if list.iter().any(|p| p.zz_mark.contains(filter.as_str())) {
                hui_list.push(c.name.clone());
                data.push(c.clone());
            }
        }
    }

    info!("{} = {:?}", filter,  hui_list);

    save_cache_with_path(&ResultOutput{
        company_list: data,
    }, "result.json");

    Ok(())

}
