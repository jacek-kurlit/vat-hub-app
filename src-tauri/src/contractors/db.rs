use crate::{contractors::Contractor, Database};

pub struct ContractorRepository {
    db: Database,
}

impl ContractorRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn save_contractor(&self, _contractor: Contractor) -> Result<(), String> {
        // Implement saving logic here
        Ok(())
    }
}
