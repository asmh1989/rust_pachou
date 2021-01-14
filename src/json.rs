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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    #[serde(rename = "Legalmanname")]
    pub legal_man_name: Option<String>,
    #[serde(rename = "OpinionDateTime")]
    pub option_date_time: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "City")]
    pub city : String,
    #[serde(rename = "CorpCode")]
    pub corp_code: String,
    #[serde(rename = "SCUCode")]
    pub scu_code: Option<String>,
    #[serde(rename = "CorpName")]
    pub name: String,

    pub qualification: Option<Vec<Qualification>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultOutput {
    pub company_list: Vec<Company>,
    pub hun_nin_tu: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Qualification {
    #[serde(rename = "APTITUDEKINDNAME")]
    pub aptitude_kind_name: String,
    #[serde(rename = "CertID")]
    pub cert_id: String,
    #[serde(rename = "Zzmark")]
    pub zz_mark: String,
    #[serde(rename = "OrganDate")]
    pub organ_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "TechMan")]
    pub tech_man: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgQualification {
    #[serde(rename = "Code")]
    pub code: u32,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Total")]
    pub total: u32,
    #[serde(rename = "Obj")]
    pub obj: Vec<Qualification>,
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
