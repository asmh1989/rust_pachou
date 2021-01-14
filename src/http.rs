use log::info;
use reqwest::header;
use reqwest::header::HeaderValue;

use crate::json::{Company, Msg, MsgQualification, Query, Query2};
use crate::result_err;
use once_cell::sync::OnceCell;

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

pub async fn get_company_qualification(company: &mut Company) -> Result<(), String> {
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

    // info!("get_company_qualification = {} ", s.clone());

    let q: MsgQualification = serde_json::from_str(&s).map_err(crate::result_err!())?;

    company.qualification = Some(q.obj);

    Ok(())
}

pub async fn get_company_list(query: &Query) -> Result<Vec<Company>, String> {
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

    let data: Msg = serde_json::from_str(&s).unwrap();

    info!("find company size = {:?} ", data.obj.len());

    // for mut c in data.obj {
    //     if need_download_qualification(&c) {
    //         let _ = get_company_qualification(&mut c).await;
    //
    //         refresh_company(&c);
    //         info!(
    //             "获取最新资质信息 code = {}, res2 = {} \n",
    //             c.corp_code.clone(),
    //             s
    //         );
    //     }
    // }

    Ok(data.obj)
}

#[cfg(test)]

mod tests {
    use crate::cache::{get_cache, save_cache, ResultOutput, get_cache_from_path, save_cache_with_path};
    use log::info;
    use crate::json::{Query, Company};
    use std::fs::File;
    use std::io::Write;

    fn add_company(data: &mut Vec<Company>, new: &Vec<Company>) {
        for c  in new  {
            if ! data.iter().any(|p| p.name == c.name) {
                data.push(c.clone());
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_company()  {
        crate::cache::init_cache();

        let mut data : Vec<Company> = Vec::new();
        let result = super::get_company_list(&Query::new_name("混凝土")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("建设")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("建材")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("海宁")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("海盐")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("浙江")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("嘉善")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("平湖")).await.unwrap();
        add_company(&mut data, &result);

        let result = super::get_company_list(&Query::new_name("桐乡")).await.unwrap();
        add_company(&mut data, &result);

        let result = ResultOutput{
            company_list: data,
        };

        let mut output: File = File::create("config/output_3.json").unwrap();
        output
            .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
            .unwrap();

    }

    #[tokio::test]
    async fn test_get_hui() {
        crate::cache::init_cache();
        let cache = get_cache();

        let mut data:Vec<Company> = Vec::new();

        let mut hui_list:Vec<String> = Vec::new();

        for c in cache.company_list  {
            if let Some(list) = c.qualification.clone() {
                if list.iter().any(|p| p.zz_mark.contains("预拌混凝土专业承包不分等级")) {
                    hui_list.push(c.name.clone());
                    data.push(c.clone());
                }
            }
        }

        info!("hui = {:?}", hui_list);

        save_cache_with_path(&ResultOutput{
            company_list: data,
        }, "config/hui.json");

    }

    #[tokio::test]
    async fn test_fetch() {
        crate::cache::init_cache();
        let cache = get_cache();
        let cache2 = get_cache_from_path("config/output_3.json");

        let mut list = cache.company_list.clone();
        let list2 = cache2.company_list.clone();

        for p in list2  {
            if !list.iter().any(|q| q.name == p.name) {
                info!("found new company {}", p.name.clone());
                list.push(p);
            }
        }

        let mut i = 0;
        for mut c in &mut list {
            if c.qualification.is_none() {
                i+=1;
                info!("{},  fetch {:?} 资质", i, c.name.clone());
                let res = super::get_company_qualification(&mut c).await;

                if res.is_err() {
                    break;
                }
            }
        }

        save_cache(&ResultOutput {
            company_list: list,
        });
    }
}
