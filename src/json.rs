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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Company {
    #[serde(rename = "CorpName")]
    pub name: String,
    #[serde(rename = "Legalmanname")]
    pub legal_man_name: Option<String>,
    #[serde(rename = "OpinionDateTime")]
    pub option_date_time: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "CorpCode")]
    pub corp_code: String,
    #[serde(rename = "SCUCode")]
    pub scu_code: Option<String>,

    pub qualification: Option<Vec<Qualification>>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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
    #[serde(rename = "TechMan", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "corpName")]
    pub corp_name: String,
    #[serde(rename = "CertID")]
    pub cert_id: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,

}

impl Default for Query {
    fn default() -> Self {
        Self {
            mark: "建筑业".to_string(),
            city: "嘉兴市".to_string(),
            page_index: 1,
            page_size: 1000,
            corp_name: "".to_string(),
            cert_id: "".to_string(),
            end_date: "".to_string()
        }
    }
}

impl Query {
    pub fn new_name(name: &str) ->Self {
        let mut d = Query::default();
        d.corp_name = name.to_string();
        d
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
