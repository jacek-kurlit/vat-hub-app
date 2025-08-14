use serde::{Deserialize, Serialize};

mod api;
#[derive(Serialize, Deserialize)]
pub struct Contractor {
    name: String,
    vat_status: String,
    regon: String,
    krs: String,
    residence_address: String,
    accounts_numbers: Vec<String>,
}

#[tauri::command]
pub async fn fetch_contractor_data(nip: String) -> Result<Contractor, String> {
    api::fetch_contractor_data(&nip).await.map(|data| {
        let subject = data.result.subject;
        Contractor {
            name: subject.name,
            vat_status: subject.status_vat,
            regon: subject.regon,
            krs: subject.krs.unwrap_or_default(),
            residence_address: subject.residence_address,
            accounts_numbers: subject.account_numbers,
        }
    })
}

#[tauri::command]
pub async fn save_contractor(contractor: Contractor) -> Result<(), String> {
    let _ = contractor;
    Ok(())
}
