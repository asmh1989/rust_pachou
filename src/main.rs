use log::info;
use log4rs::init_file;

use reqwest::header;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

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
pub struct Company {
    #[serde(rename = "CorpCode")]
    pub corp_code: String,
    #[serde(rename = "SCUCode")]
    pub scu_code: String,
    #[serde(rename = "CorpName")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    #[serde(rename = "Code")]
    pub code: u32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Total")]
    pub total: u32,
    #[serde(rename = "Obj")]
    pub obj: Vec<Company>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "Zzmark")]
    pub mark: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "pageIndex")]
    pub page_index: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            mark: "建筑业".to_string(),
            city: "嘉兴市".to_string(),
            page_index: 1,
            page_size: 1000,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query2 {
    #[serde(rename = "CORPCODE")]
    pub code: String,
    #[serde(rename = "zzType")]
    pub z_type: u32,
}

impl Query2 {
    pub fn new(code: String) -> Self {
        Self { code, z_type: 1 }
    }
}

pub async fn get_company_list() -> Result<(), String> {
    let url = "http://223.4.70.240/ZJJGManagerWebApi/api/EnterpriseInfo/GetEnterpriseInfo";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36 Edg/87.0.664.75"),
    );
    headers.insert(header::HOST, HeaderValue::from_static("223.4.70.240"));
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );

    headers.insert(
        header::ORIGIN,
        HeaderValue::from_static("http://223.4.65.131:8080"),
    );

    headers.insert(
        header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate"),
    );

    headers.insert(
        header::REFERER,
        HeaderValue::from_static("http://223.4.65.131:8080/"),
    );

    // headers.insert(header::CONTENT_LENGTH, HeaderValue::from_static("118"));

    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .headers(headers.clone())
        .form(&Query::default())
        .send()
        .await
        .map_err(result_err!())
        .unwrap();

    let s = { res.text().await.unwrap() }
        .replace("\\", "")
        .replace("\"{", "{")
        .replace("}\"", "}");
    // info!("res = {} \n", s);

    let data: Msg = serde_json::from_str(&s).unwrap();

    // info!("data = {:?} ", data);

    let url2 =
        "http://223.4.70.240/ZJJGManagerWebApi/api/EnterpriseInfo/GetEnterpriseQualificationNew";

    let mut my_vec: Vec<String> = Vec::new();

    for c in data.obj {
        let res = client
            .post(url2)
            .headers(headers.clone())
            .form(&Query2::new(c.corp_code.clone()))
            .send()
            .await
            .map_err(result_err!())
            .unwrap();

        let s = { res.text().await.unwrap() }
            .replace("\\", "")
            .replace("\"{", "{")
            .replace("}\"", "}");

        if s.contains("预拌混凝土专业承包不分等级") {
            my_vec.push(c.name.clone());
        }

        // info!("code = {}, res2 = {} \n", c.corp_code.clone(), s);
    }

    info!("find need company : {:?}", my_vec);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    init_file("config/log4rs.yaml", Default::default()).unwrap();

    let _ = get_company_list().await;

    info!("Hello, world!");

    Ok(())
}
