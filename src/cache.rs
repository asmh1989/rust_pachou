use std::fs::File;
use crate::json::{Msg, ResultOutput, Company};
use std::io::{BufReader, Write};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::borrow::Borrow;
use std::ops::Deref;
use reqwest::header;
use log4rs::init_file;
use reqwest::header::HeaderValue;
use crate::http::init_header;

static RESULT_OUTPUT:OnceCell<ResultOutput>  = OnceCell::new();


pub fn init_cache() {
    init_file("config/log4rs.yaml", Default::default()).unwrap();

    let file = File::open("config/output.json").unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: ResultOutput = serde_json::from_reader(reader).unwrap();

    let _ = RESULT_OUTPUT.set(u);

    init_header();
}

pub fn save_cache() {
    let mut output: File = File::create("config/output.json").unwrap();
    let result = RESULT_OUTPUT.get().unwrap().clone();
    output.write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes()).unwrap();
}

pub fn set_result(result: &ResultOutput) {
    let _ = RESULT_OUTPUT.set(result.clone());
}

pub fn get_cache() -> &'static ResultOutput {
    RESULT_OUTPUT.get().expect("result not init")
}

pub fn need_download_qualification(company: &Company) -> bool {
    let cache = RESULT_OUTPUT.get().expect("result not init").clone();
    for c in cache.company_list.clone() {

        if c.name == company.name && c.qualification.is_some() {
            return false;
        }
    }

    true
}

pub fn refresh_company(company: &Company) {
    let mut cache = RESULT_OUTPUT.get().expect("result not init").clone();
    for mut c in &mut cache.company_list {

        if c.name == company.name {
            c.qualification = company.qualification.clone();
            return;
        }
    }

    cache.company_list.push(company.clone());

}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Write};
    use crate::json::{Msg, ResultOutput};
    use log4rs::init_file;
    use log::info;

    #[tokio::test]
    async fn test_company() {

        let file = File::open("config/jixing.json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u: Msg = serde_json::from_reader(reader).unwrap();

        info!("msg = {:?} \n", u);

        let mut output: File = File::create("config/output.json").unwrap();
        let mut result = ResultOutput {
            company_list: u.obj,
            hun_nin_tu: Vec::new()
        };

        output.write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes()).unwrap();
    }


    #[tokio::test]
    async fn test_cache() {
        super::init_cache();
        // super::save_cache();
    }


}