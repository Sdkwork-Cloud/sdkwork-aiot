//! Transactional outbox read/publish port for `iot_outbox_event`.

/// Pending outbox row (`status = 0`).
pub const OUTBOX_STATUS_PENDING: i64 = 0;
/// Successfully published to downstream consumers.
pub const OUTBOX_STATUS_PUBLISHED: i64 = 1;
/// Permanently failed after max publish attempts.
pub const OUTBOX_STATUS_FAILED: i64 = 2;
/// Claimed by an outbox dispatcher worker and awaiting publish acknowledgement.
pub const OUTBOX_STATUS_CLAIMED: i64 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotOutboxPendingEvent {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub event_id: String,
    pub event_type: String,
    pub event_version: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub payload_json: String,
    pub trace_id: Option<String>,
    pub attempt_count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutboxEventRepositoryError {
    PersistenceFailure,
    NotFound,
}

pub trait OutboxEventRepository: Send + Sync {
    fn pending_lag_count(&self) -> u64;

    fn claim_pending_batch(
        &self,
        limit: usize,
    ) -> Result<Vec<AiotOutboxPendingEvent>, OutboxEventRepositoryError>;

    fn mark_published(
        &self,
        tenant_id: i64,
        event_id: &str,
    ) -> Result<(), OutboxEventRepositoryError>;

    fn record_publish_failure(
        &self,
        tenant_id: i64,
        event_id: &str,
        max_attempts: u32,
    ) -> Result<(), OutboxEventRepositoryError>;
}
