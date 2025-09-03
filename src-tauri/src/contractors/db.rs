use std::collections::HashMap;

use sqlx::Acquire;

use crate::{contractors::Contractor, Database};

pub struct ContractorRepository {
    db: Database,
}

#[derive(sqlx::FromRow)]
struct ContractorRow {
    id: i64,
    name: String,
    nip: String,
    vat_status: String,
    regon: Option<String>,
    krs: Option<String>,
    residence_address: Option<String>,
    working_address: Option<String>,
    account_number: Option<String>,
}

impl ContractorRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn save_contractor(&self, contractor: Contractor) -> Result<(), String> {
        //TODO: make nip unique to prevent duplicate
        let mut conn = self
            .db
            .pool
            .acquire()
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;
        let mut trans = conn
            .begin()
            .await
            .map_err(|e| format!("Error starting transaction: {}", e))?;

        let result = sqlx::query!(
            "INSERT INTO contractors (name, vat_status, nip, regon, krs, residence_address, working_address) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
            contractor.name,
            contractor.vat_status,
            contractor.nip,
            contractor.regon,
            contractor.krs,
            contractor.residence_address,
            contractor.working_address
        ).fetch_one(trans.as_mut()).await.map_err(|e| format!("Error saving contractor: {}", e))?;
        // Implement saving logic here
        for account_number in contractor.accounts_numbers {
            sqlx::query!(
                "INSERT INTO account_numbers (contractor_id, account_number) VALUES ($1, $2)",
                result.id,
                account_number
            )
            .execute(trans.as_mut())
            .await
            .map_err(|e| format!("Error saving account number: {}", e))?;
        }
        trans
            .commit()
            .await
            .map_err(|e| format!("Error committing transaction: {}", e))?;
        Ok(())
    }

    pub async fn fetch_contractors(
        &self,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<Contractor>, String> {
        let size = page_size as i64;
        let offset = (page.saturating_sub(1) * page_size) as i64;

        let rows = sqlx::query_as!(
            ContractorRow,
            r#"
            SELECT
                c.id, c.name, c.nip, c.vat_status, c.regon, c.krs,
                c.residence_address, c.working_address, an.account_number
            FROM (
                SELECT id FROM contractors ORDER BY id LIMIT ? OFFSET ?
            ) AS page
            JOIN contractors c ON c.id = page.id
            LEFT JOIN account_numbers an ON c.id = an.contractor_id
            ORDER BY c.id;
            "#,
            size,
            offset
        )
        .fetch_all(&self.db.pool) // Assuming a `pool` field on your `Database` struct
        .await
        .map_err(|e| e.to_string())?;

        let mut contractors: HashMap<i64, Contractor> = HashMap::new();

        for row in rows {
            let contractor = contractors.entry(row.id).or_insert_with(|| Contractor {
                name: row.name,
                nip: row.nip,
                vat_status: row.vat_status,
                regon: row.regon.unwrap_or_default(),
                krs: row.krs.unwrap_or_default(),
                residence_address: row.residence_address,
                working_address: row.working_address,
                accounts_numbers: Vec::new(),
            });

            if let Some(account_number) = row.account_number {
                contractor.accounts_numbers.push(account_number);
            }
        }

        Ok(contractors.into_values().collect())
    }
}
