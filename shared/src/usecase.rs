use async_trait::async_trait;

#[async_trait]
pub trait Usecase<I, O, E>: Send + Sync {
    async fn execute(&self, input: I) -> Result<O, E>;
}
