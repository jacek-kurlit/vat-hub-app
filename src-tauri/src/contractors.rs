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
        self.api_client
            .fetch_contractor_data(&nip)
            .await?
            .ok_or_else(|| "Nie znaleziono kontrahenta".to_string())
    }

    pub async fn fetch_contractors(
        &self,
        page: usize,
        page_size: usize,
        search: Option<String>,
    ) -> Result<Vec<Contractor>, String> {
        self.repo.fetch_contractors(page, page_size, search).await
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
    krs: Option<String>,
    residence_address: Option<String>,
    working_address: Option<String>,
    accounts_numbers: Vec<String>,
}

//TODO: unify error handling strategy
#[tauri::command]
pub async fn fetch_contractor_data(
    nip: String,
    service: State<'_, ContractorService>,
) -> Result<Contractor, String> {
    service.fetch_contractor_data(nip).await
}

#[tauri::command]
pub async fn save_contractor(
    contractor: Contractor,
    service: State<'_, ContractorService>,
) -> Result<(), String> {
    service.save_contractor(contractor).await
}

//TODO: add missing tests
//TODO: add sorting
#[tauri::command]
pub async fn fetch_contractors(
    page: usize,
    page_size: usize,
    search: Option<String>,
    service: State<'_, ContractorService>,
) -> Result<Vec<Contractor>, String> {
    service.fetch_contractors(page, page_size, search).await
}
