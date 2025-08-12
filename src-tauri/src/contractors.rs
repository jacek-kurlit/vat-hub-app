use serde::Serialize;
#[derive(Serialize)]
pub struct Contractor {
    name: String,
    vat_status: String,
    regon: String,
    krs: String,
    residence_address: String,
    accounts_numbers: Vec<String>,
}

#[tauri::command]
pub async fn fetch_contractor_data() -> Contractor {
    Contractor {
        name: "Przykładowa Firma Sp. z o.o.".to_string(),
        vat_status: "Aktywny".to_string(),
        regon: "123456789".to_string(),
        krs: "0000123456".to_string(),
        residence_address: "ul. Przykładowa 123\n00-001 Warszawa\nPolska".to_string(),
        accounts_numbers: vec![
            "12 3456 7890 1234 5678 9012 3456".to_string(),
            "98 7654 3210 9876 5432 1098 7654".to_string(),
        ],
    }
}
