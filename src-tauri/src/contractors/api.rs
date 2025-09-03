use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FetchContractorResponse {
    pub result: ResultData,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ResultData {
    pub subject: Option<Subject>,
    #[serde(rename = "requestDateTime")]
    pub request_date_time: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Subject {
    pub regon: String,
    #[serde(rename = "statusVat")]
    pub status_vat: String,
    pub krs: Option<String>,
    #[serde(rename = "accountNumbers")]
    pub account_numbers: Vec<String>,
    pub nip: String,
    pub name: String,
    #[serde(rename = "residenceAddress")]
    pub residence_address: Option<String>,
    #[serde(rename = "workingAddress")]
    pub working_address: Option<String>,
    #[serde(rename = "registrationLegalDate")]
    pub registration_legal_date: chrono::NaiveDate,
}

pub struct ContracorApiClient {
    client: reqwest::Client,
}

impl ContracorApiClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_contractor_data(
        &self,
        nip: &str,
    ) -> Result<FetchContractorResponse, String> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let url = format!(
            "https://wl-api.mf.gov.pl/api/search/nip/{}?date={}",
            nip, today
        );
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<FetchContractorResponse>()
            .await
            .map_err(|e| e.to_string())
    }
}
