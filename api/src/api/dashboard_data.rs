use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DashboardData {
    pub repo_count: i64,
    pub storage: i64,
}

#[derive(Default)]
pub struct DashboardDataBuilder {
    pub data: DashboardData,
}

impl DashboardDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn repo_count(&mut self, repo_count: i64) -> &mut Self {
        self.data.repo_count = repo_count;
        self
    }

    pub fn storage(&mut self, storage: i64) -> &mut Self {
        self.data.storage = storage;
        self
    }

    pub fn build(&self) -> DashboardData {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::dashboard_data::DashboardDataBuilder;

    #[test]
    fn test_builder() {
        let data = DashboardDataBuilder::new()
            .repo_count(5)
            .storage(100)
            .build();

        assert_eq!(data.repo_count, 5);
        assert_eq!(data.storage, 100);
    }
}
