use sqlx::Acquire;

use crate::{contractors::Contractor, Database};

pub struct ContractorRepository {
    db: Database,
}

#[derive(sqlx::FromRow)]
struct ContractorRow {
    name: String,
    nip: String,
    vat_status: String,
    regon: String,
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
        search: Option<String>,
    ) -> Result<Vec<Contractor>, String> {
        let size = page_size as i64;
        let offset = (page.saturating_sub(1) * page_size) as i64;
        let search_pattern = search
            .map(|s| format!("%{}%", s))
            .unwrap_or("%%".to_string());

        let rows = sqlx::query_as!(
            ContractorRow,
            r#"
            SELECT
                c.name, c.nip, c.vat_status, c.regon, c.krs,
                c.residence_address, c.working_address, an.account_number
            FROM (
                SELECT id FROM contractors WHERE nip LIKE ? or name LIKE ? ORDER BY id LIMIT ? OFFSET ?
            ) AS page
            JOIN contractors c ON c.id = page.id
            LEFT JOIN account_numbers an ON c.id = an.contractor_id
            ORDER BY c.id;
            "#,
            search_pattern,
            search_pattern,
            size,
            offset
        )
        .fetch_all(&self.db.pool) // Assuming a `pool` field on your `Database` struct
        .await
        .map_err(|e| e.to_string())?;

        let size = rows.len();
        let res =
            rows.into_iter()
                .fold(Vec::with_capacity(size), |mut acc: Vec<Contractor>, row| {
                    match acc.last_mut() {
                        Some(contractor) => {
                            if contractor.nip == row.nip && row.account_number.is_some() {
                                contractor
                                    .accounts_numbers
                                    .push(row.account_number.unwrap());
                            } else {
                                acc.push(Contractor::from(row));
                            }
                        }
                        None => {
                            acc.push(Contractor::from(row));
                        }
                    };
                    acc
                });
        Ok(res)
    }
}

impl From<ContractorRow> for Contractor {
    fn from(value: ContractorRow) -> Self {
        Contractor {
            name: value.name,
            nip: value.nip,
            vat_status: value.vat_status,
            regon: value.regon,
            krs: value.krs,
            residence_address: value.residence_address,
            working_address: value.working_address,
            accounts_numbers: value.account_number.map_or(vec![], |n| vec![n]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqliteConnectOptions;

    #[tokio::test]
    async fn test_save_contractor_success() {
        let repo = ContractorRepository::test_repository().await;
        let contractor = Contractor::fixture();

        let result = repo.save_contractor(contractor).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_contractor_with_optional_fields_none() {
        let repo = ContractorRepository::test_repository().await;
        let contractor = Contractor {
            krs: None,
            residence_address: None,
            working_address: None,
            accounts_numbers: vec![],
            ..Contractor::fixture()
        };

        let result = repo.save_contractor(contractor).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_contractors_empty_database() {
        let repo = ContractorRepository::test_repository().await;

        let result = repo.fetch_contractors(1, 10, None).await;
        assert!(result.is_ok());
        let contractors = result.unwrap();
        assert_eq!(contractors.len(), 0);
    }

    #[tokio::test]
    async fn test_fetch_contractors_with_data() {
        let repo = ContractorRepository::test_repository().await;

        // Save a test contractor
        let contractor = Contractor::fixture();
        repo.save_contractor(contractor.clone()).await.unwrap();

        let result = repo.fetch_contractors(1, 10, None).await;
        let contractors = result.unwrap();
        assert_eq!(contractors.len(), 1);

        let fetched = &contractors[0];
        assert_eq!(fetched, &contractor);
    }

    #[tokio::test]
    async fn test_fetch_contractors_pagination() {
        let repo = ContractorRepository::test_repository().await;

        // Save multiple contractors
        for i in 1..=5 {
            let contractor = Contractor {
                name: format!("Company {}", i),
                nip: format!("123456789{}", i),
                ..Contractor::fixture()
            };
            repo.save_contractor(contractor).await.unwrap();
        }

        // Test first page
        let result = repo.fetch_contractors(1, 2, None).await;
        assert!(result.is_ok_and(|v| v.len() == 2));

        // Test second page
        let result = repo.fetch_contractors(2, 2, None).await;
        assert!(result.is_ok_and(|v| v.len() == 2));

        // Test third page
        let result = repo.fetch_contractors(3, 2, None).await;
        assert!(result.is_ok_and(|v| v.len() == 1));
    }

    #[tokio::test]
    async fn test_fetch_contractors_search_by_name() {
        let repo = ContractorRepository::test_repository().await;

        // Save contractors with different names
        let contractor1 = Contractor {
            name: "ABC Company".to_string(),
            nip: "1111111111".to_string(),
            ..Contractor::fixture()
        };

        let contractor2 = Contractor {
            name: "XYZ Corporation".to_string(),
            nip: "2222222222".to_string(),
            ..Contractor::fixture()
        };

        repo.save_contractor(contractor1).await.unwrap();
        repo.save_contractor(contractor2).await.unwrap();

        // Search for "ABC"
        let result = repo.fetch_contractors(1, 10, Some("ABC".to_string())).await;
        assert!(result.is_ok());
        let contractors = result.unwrap();
        assert_eq!(contractors.len(), 1);
        assert_eq!(contractors[0].name, "ABC Company");
    }

    #[tokio::test]
    async fn test_fetch_contractors_search_by_nip() {
        let repo = ContractorRepository::test_repository().await;

        let contractor = Contractor {
            nip: "5555555555".to_string(),
            ..Contractor::fixture()
        };

        repo.save_contractor(contractor).await.unwrap();

        // Search by partial NIP
        let result = repo
            .fetch_contractors(1, 10, Some("5555".to_string()))
            .await;
        assert!(result.is_ok());
        let contractors = result.unwrap();
        assert_eq!(contractors.len(), 1);
        assert_eq!(contractors[0].nip, "5555555555");
    }

    #[tokio::test]
    async fn test_fetch_contractors_no_search_results() {
        let repo = ContractorRepository::test_repository().await;

        let contractor = Contractor::fixture();
        repo.save_contractor(contractor).await.unwrap();

        // Search for something that doesn't exist
        let result = repo
            .fetch_contractors(1, 10, Some("NonExistent".to_string()))
            .await;
        assert!(result.is_ok());
        let contractors = result.unwrap();
        assert_eq!(contractors.len(), 0);
    }

    impl Contractor {
        fn fixture() -> Contractor {
            Contractor {
                name: "Test Company".to_string(),
                nip: "1234567890".to_string(),
                vat_status: "Active".to_string(),
                regon: "123456789".to_string(),
                krs: Some("0000123456".to_string()),
                residence_address: Some("Test Address 1".to_string()),
                working_address: Some("Test Address 2".to_string()),
                accounts_numbers: vec![
                    "12345678901234567890123456".to_string(),
                    "98765432109876543210987654".to_string(),
                ],
            }
        }
    }

    impl Database {
        async fn test_database() -> Database {
            let connection_options = SqliteConnectOptions::new()
                .in_memory(true)
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Memory);

            let pool = sqlx::pool::PoolOptions::new()
                //NOTE: For in-memory database, max_connections must be 1 otherwise other connections won't have migrations applied
                .max_connections(1)
                .connect_with(connection_options)
                .await
                .unwrap();

            // Run migrations
            sqlx::migrate!("./migrations").run(&pool).await.unwrap();

            Database { pool }
        }
    }

    impl ContractorRepository {
        async fn test_repository() -> ContractorRepository {
            let db = Database::test_database().await;
            ContractorRepository::new(db)
        }
    }
}
