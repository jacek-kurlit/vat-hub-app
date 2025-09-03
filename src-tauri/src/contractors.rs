use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::State;

mod api;
mod db;

pub struct ContractorService {
    repo: db::ContractorRepository,
    api_client: api::ContracorApiClient,
}

impl ContractorService {
    pub fn new(db: crate::Database, client: Client) -> Self {
        Self {
            repo: db::ContractorRepository::new(db),
            api_client: api::ContracorApiClient::new(client),
        }
    }

    pub async fn fetch_contractor_data(&self, nip: String) -> Result<Contractor, String> {
        let subject = self
            .api_client
            .fetch_contractor_data(&nip)
            .await?
            .result
            .subject
            .ok_or_else(|| "Nie znaleziono kontrahenta".to_string())?;
        Ok(Contractor {
            name: subject.name,
            nip: subject.nip,
            vat_status: subject.status_vat,
            regon: subject.regon,
            krs: subject.krs.unwrap_or_default(),
            residence_address: subject.residence_address,
            working_address: subject.working_address,
            accounts_numbers: subject.account_numbers,
        })
    }

    pub async fn fetch_contractors(
        &self,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<Contractor>, String> {
        self.repo.fetch_contractors(page, page_size).await
    }

    pub async fn save_contractor(&self, contractor: Contractor) -> Result<(), String> {
        self.repo.save_contractor(contractor).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct Contractor {
    name: String,
    nip: String,
    vat_status: String,
    regon: String,
    krs: String,
    residence_address: Option<String>,
    working_address: Option<String>,
    accounts_numbers: Vec<String>,
}

#[tauri::command]
pub async fn fetch_contractor_data(
    nip: String,
    service: State<'_, ContractorService>,
) -> Result<Contractor, String> {
    //TODO: this method may map errors to FE friendly format
    service.fetch_contractor_data(nip).await
}

#[tauri::command]
pub async fn save_contractor(
    contractor: Contractor,
    service: State<'_, ContractorService>,
) -> Result<(), String> {
    service.save_contractor(contractor).await
}

#[tauri::command]
pub async fn fetch_contractors(
    page: usize,
    page_size: usize,
    service: State<'_, ContractorService>,
) -> Result<Vec<Contractor>, String> {
    service.fetch_contractors(page, page_size).await
}
