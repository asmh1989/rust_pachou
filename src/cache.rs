use crate::http::init_header;

use serde::{Deserialize, Serialize};

use crate::json::Company;
use log4rs::init_file;
use std::{
    fs::File,
    io::{BufReader, Write},
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultOutput {
    pub company_list: Vec<Company>,
}

impl ResultOutput {
    pub fn set_company_list(&mut self, list: &Vec<Company>) {
        self.company_list = list.clone();
    }
}

pub fn get_cache_from_path(path: &str) -> ResultOutput {

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    serde_json::from_reader(reader).unwrap()

}

pub fn get_cache()-> ResultOutput {
    get_cache_from_path("config/output.json")
}

pub fn init_cache() {
    init_file("config/log4rs.yaml", Default::default()).unwrap();

    // let file = File::open("config/output.json").unwrap();
    // let reader = BufReader::new(file);
    //
    // // Read the JSON contents of the file as an instance of `User`.
    // let u: ResultOutput = serde_json::from_reader(reader).unwrap();
    //
    // let _ = RESULT_OUTPUT.set(u);

    init_header();
}

pub fn save_cache(result: &ResultOutput) {
    save_cache_with_path(result, "config/output.json");
}

pub fn save_cache_with_path(result: &ResultOutput, path: &str) {
    let mut output: File = File::create(path).unwrap();
    output
        .write_all(serde_json::to_string_pretty(result).unwrap().as_bytes())
        .unwrap();
}


#[cfg(test)]
mod tests {
    use crate::json::Msg;
    use log::info;
    use std::fs::File;
    use std::io::{BufReader, Write};

    use super::ResultOutput;

    #[tokio::test]
    async fn test_company() {
        let file = File::open("config/jixing.json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u: Msg = serde_json::from_reader(reader).unwrap();

        info!("msg = {:?} \n", u);

        let mut output: File = File::create("config/output.json").unwrap();
        let result = ResultOutput {
            company_list: u.obj,
        };

        output
            .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
            .unwrap();
    }

    #[tokio::test]
    async fn test_cache() {
        super::init_cache();
        // super::save_cache();
    }
}
