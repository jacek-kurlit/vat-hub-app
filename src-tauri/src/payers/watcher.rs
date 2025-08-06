use anyhow::Result;

use crate::Database;

#[derive(Clone)]
#[allow(dead_code)]
pub struct WatcherService {
    repo: WatcherRepo,
}

#[derive(Clone)]
#[allow(dead_code)]
struct WatcherRepo {
    db: Database,
}

impl WatcherService {
    pub fn new(db: Database) -> Self {
        Self {
            repo: WatcherRepo { db },
        }
    }

    pub async fn fetch_watchers(&self, _page: usize, _limit: usize) -> Result<Vec<WatchedPayer>> {
        todo!()
    }

    pub async fn import_payers(&self) -> Result<()> {
        todo!();
    }
}

pub struct WatchedPayer {
    pub vat_id: String,
}
