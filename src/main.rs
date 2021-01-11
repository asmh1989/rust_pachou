use log4rs::init_file;

use serde::{Deserialize, Serialize};
use reqwest::header;
use reqwest::header::HeaderValue;

#[macro_export]
macro_rules! result_err {
    () => {
        |err| {
            info!("err = {}", err);
            format!("{:?}", err)
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company{
    #[serde(rename = "CorpCode")]
    pub corp_code: String,
    #[serde(rename = "SCUCode")]
    pub scu_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildParams {
    #[serde(rename = "Code")]
    pub code: u32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Total")]
    pub total: u32,
    #[serde(rename = "Obj")]
    pub obj: Vec<Company>,
}


pub async fn get_company_list() -> Result<(), String>{

    let url = "http://223.4.70.240/ZJJGManagerWebApi/api/EnterpriseInfo/GetEnterpriseInfo";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36 Edg/87.0.664.75"),
    );
    headers.insert(
        header::HOST,
        HeaderValue::from_static("223.4.70.240"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("*/*"),
    );

    headers.insert(
        header::ORIGIN,
        HeaderValue::from_static("http://223.4.65.131:8080"),
    );

    headers.insert(
        header::REFERER,
        HeaderValue::from_static("http://223.4.65.131:8080/"),
    );

    let client = reqwest::Client::new();

    let form = "CertID=&EndDate=&Zzmark=%E5%BB%BA%E7%AD%91%E4%B8%9A&City=%E5%98%89%E5%85%B4%E5%B8%82&pageIndex=1&pageSize=100";

    let res = client.post(url).headers(headers).form(form).send().await.map_err(result_err!());

    let s = { result.text().await.unwrap() };
    info!("res ", s);


    Ok(())

}

#[tokio::main]
fn main() {
    init_file("config/log4rs.yaml", Default::default()).unwrap();

    let  _ = get_company_list().await;

    info!("Hello, world!");
}
