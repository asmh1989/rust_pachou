use reqwest::{header, Response};
use reqwest::header::HeaderValue;
use log::info;

use tokio::time::Duration;
use crate::json::{Query, Msg, Query2, Company, Qualification, MsgQualification};
use once_cell::sync::OnceCell;
use crate::cache::{need_download_qualification, refresh_company};
use crate::result_err;

static HEADERS: OnceCell<header::HeaderMap> = OnceCell::new();

pub fn init_header() {
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

    let _ = HEADERS.set(headers);
}

pub async fn get_company_qualification(company: &mut Company) ->Result<(), String> {

    let client = reqwest::Client::new();

    let headers = HEADERS.get().expect("header not init");

    let url =
        "http://223.4.70.240/ZJJGManagerWebApi/api/EnterpriseInfo/GetEnterpriseQualificationNew";
    let res = client
        .post(url)
        .headers(headers.clone())
        .form(&Query2::new(company.corp_code.clone()))
        .send()
        .await
        .map_err(crate::result_err!())?;

    let s = { res.text().await.unwrap() }
        .replace("\\", "")
        .replace("\"{", "{")
        .replace("}\"", "}");

    info!("get_company_qualification = {} ", s.clone());

    let q :MsgQualification = serde_json::from_str(&s).map_err(crate::result_err!())?;

    company.qualification = Some(q.obj);

    Ok(())
}

pub async fn get_company_list(query: &Query) -> Result<(), String> {
    let url = "http://223.4.70.240/ZJJGManagerWebApi/api/EnterpriseInfo/GetEnterpriseInfo";

    // headers.insert(header::CONTENT_LENGTH, HeaderValue::from_static("118"));

    let client = reqwest::Client::new();

    let headers = HEADERS.get().expect("header not init");


    let res = client
        .post(url)
        .headers(headers.clone())
        .form(query)
        .send()
        .await
        .map_err(result_err!())?;

    let s = { res.text().await.unwrap() }
        .replace("\\", "")
        .replace("\"{", "{")
        .replace("}\"", "}");
    // info!("res = {} \n", s);

    let mut data: Msg = serde_json::from_str(&s).unwrap();

    info!("find company size = {:?} ", data.obj.len());

    let mut i= 0;
    for mut c in data.obj {
        i+=1;
        if need_download_qualification(&c) {
            let _ = get_company_qualification(&mut c).await;

            refresh_company(&c);
            info!("获取最新资质信息 code = {}, res2 = {} \n", c.corp_code.clone(), s);
        }
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::cache::{get_cache, save_cache, set_result};
    use log::info;

    #[tokio::test]
    async fn test_fetch() {
        crate::cache::init_cache();
        let mut cache = get_cache().clone();

        for mut c in  &mut cache.company_list{
            if c.qualification.is_none() {
                info!("fetch {:?} 资质", c.name.clone());
                let res = super::get_company_qualification(&mut c).await;

                // if res.is_err() {
                    break;
                // }
            }
        }

        set_result(&cache);

        save_cache();
    }
}
