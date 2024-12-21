use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

// 簡易的な自作DIコンテナ
#[derive(Clone)]
pub struct AppRegistry {
    health_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(db: ConnectionPool) -> Self {
        let health_repository = Arc::new(HealthCheckRepositoryImpl::new(db));
        Self { health_repository }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_repository.clone()
    }
}
