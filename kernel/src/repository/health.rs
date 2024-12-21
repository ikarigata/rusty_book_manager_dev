use async_trait::async_trait;

#[async_trait]
// axumを使う場合はトレイトはSendトレイトとSyncトレイトを実装させる
pub trait HealthCheckRepository: Send + Sync {
    async fn check_db(&self) -> bool;
}
