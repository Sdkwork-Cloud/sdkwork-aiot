use std::sync::Arc;

use sdkwork_aiot_storage::{AiotOutboxPendingEvent, OutboxEventRepository};

/// Publishes a claimed outbox event to downstream consumers.
pub trait OutboxEventPublisher: Send + Sync {
    fn publish(&self, event: &AiotOutboxPendingEvent) -> Result<(), String>;
}

/// Structured stderr publisher used as the default outbox sink in standalone deployments.
#[derive(Debug, Default, Clone, Copy)]
pub struct StructuredLogOutboxPublisher;

impl OutboxEventPublisher for StructuredLogOutboxPublisher {
    fn publish(&self, event: &AiotOutboxPendingEvent) -> Result<(), String> {
        eprintln!(
            r#"{{"type":"iot.outbox.published","tenantId":{},"eventId":"{}","eventType":"{}","aggregateType":"{}","aggregateId":"{}"}}"#,
            event.tenant_id,
            json_escape(&event.event_id),
            json_escape(&event.event_type),
            json_escape(&event.aggregate_type),
            json_escape(&event.aggregate_id),
        );
        Ok(())
    }
}

/// HTTP webhook publisher for production outbox delivery (`SDKWORK_AIOT_OUTBOX_WEBHOOK_URL`).
#[derive(Debug, Clone)]
pub struct WebhookOutboxPublisher {
    url: String,
}

impl WebhookOutboxPublisher {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

impl OutboxEventPublisher for WebhookOutboxPublisher {
    fn publish(&self, event: &AiotOutboxPendingEvent) -> Result<(), String> {
        let trace_id_json = match event.trace_id.as_deref() {
            Some(value) => format!("\"{}\"", json_escape(value)),
            None => "null".to_string(),
        };
        let body = format!(
            concat!(
                r#"{{"tenantId":{},"organizationId":{},"eventId":"{}","eventType":"{}","#,
                r#""eventVersion":"{}","aggregateType":"{}","aggregateId":"{}","#,
                r#""payload":{},"traceId":{}}}"#
            ),
            event.tenant_id,
            event.organization_id,
            json_escape(&event.event_id),
            json_escape(&event.event_type),
            json_escape(&event.event_version),
            json_escape(&event.aggregate_type),
            json_escape(&event.aggregate_id),
            event.payload_json,
            trace_id_json,
        );
        blocking_http_post_json(&self.url, &body)
    }
}

const ENV_OUTBOX_WEBHOOK_URL: &str = "SDKWORK_AIOT_OUTBOX_WEBHOOK_URL";
const ENV_OUTBOX_WEBHOOK_AUTH: &str = "SDKWORK_AIOT_OUTBOX_WEBHOOK_AUTH";

pub fn outbox_publisher_from_env() -> Arc<dyn OutboxEventPublisher> {
    if let Ok(url) = std::env::var(ENV_OUTBOX_WEBHOOK_URL) {
        let url = url.trim().to_string();
        if !url.is_empty() {
            return Arc::new(WebhookOutboxPublisher::new(url));
        }
    }
    Arc::new(StructuredLogOutboxPublisher)
}

#[derive(Debug, Clone)]
pub struct AiotOutboxDispatcherConfig {
    pub batch_size: usize,
    pub max_attempts: u32,
}

impl Default for AiotOutboxDispatcherConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            max_attempts: 12,
        }
    }
}

pub struct AiotOutboxDispatcher<R: OutboxEventRepository> {
    repository: Arc<R>,
    publisher: Arc<dyn OutboxEventPublisher>,
    config: AiotOutboxDispatcherConfig,
}

impl<R: OutboxEventRepository> AiotOutboxDispatcher<R> {
    pub fn new(
        repository: Arc<R>,
        publisher: Arc<dyn OutboxEventPublisher>,
        config: AiotOutboxDispatcherConfig,
    ) -> Self {
        Self {
            repository,
            publisher,
            config,
        }
    }

    pub fn pending_lag_count(&self) -> u64 {
        self.repository.pending_lag_count()
    }

    pub fn run_once(&self) -> usize {
        let Ok(batch) = self.repository.claim_pending_batch(self.config.batch_size) else {
            return 0;
        };
        let mut published = 0_usize;
        for event in batch {
            match self.publisher.publish(&event) {
                Ok(()) => {
                    if self
                        .repository
                        .mark_published(event.tenant_id, &event.event_id)
                        .is_ok()
                    {
                        published += 1;
                    }
                }
                Err(_) => {
                    let _ = self.repository.record_publish_failure(
                        event.tenant_id,
                        &event.event_id,
                        self.config.max_attempts,
                    );
                }
            }
        }
        published
    }
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn blocking_http_post_json(url: &str, body: &str) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("outbox webhook must use http:// or https://".to_string());
    }

    let mut request = ureq::post(url)
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10));
    if let Some(auth) = outbox_webhook_auth_from_env() {
        request = request.set("Authorization", &auth);
    }

    let response = request
        .send_string(body)
        .map_err(|error| error.to_string())?;
    let status = response.status();
    if (200..300).contains(&status) {
        Ok(())
    } else {
        Err(format!("outbox webhook rejected request: HTTP {status}"))
    }
}

fn outbox_webhook_auth_from_env() -> Option<String> {
    std::env::var(ENV_OUTBOX_WEBHOOK_AUTH)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use sdkwork_aiot_storage::{
        OutboxEventRepositoryError, OUTBOX_STATUS_PENDING, OUTBOX_STATUS_PUBLISHED,
    };

    use super::*;

    #[derive(Default)]
    struct InMemoryOutboxRepo {
        events: Mutex<Vec<AiotOutboxPendingEvent>>,
        statuses: Mutex<Vec<(i64, String, i64)>>,
    }

    impl OutboxEventRepository for InMemoryOutboxRepo {
        fn pending_lag_count(&self) -> u64 {
            self.events.lock().unwrap().len() as u64
        }

        fn claim_pending_batch(
            &self,
            limit: usize,
        ) -> Result<Vec<AiotOutboxPendingEvent>, OutboxEventRepositoryError> {
            let mut events = self.events.lock().unwrap();
            let take = limit.min(events.len());
            let claimed = events.drain(..take).collect();
            Ok(claimed)
        }

        fn mark_published(
            &self,
            tenant_id: i64,
            event_id: &str,
        ) -> Result<(), OutboxEventRepositoryError> {
            self.statuses.lock().unwrap().push((
                tenant_id,
                event_id.to_string(),
                OUTBOX_STATUS_PUBLISHED,
            ));
            Ok(())
        }

        fn record_publish_failure(
            &self,
            tenant_id: i64,
            event_id: &str,
            _max_attempts: u32,
        ) -> Result<(), OutboxEventRepositoryError> {
            self.statuses.lock().unwrap().push((
                tenant_id,
                event_id.to_string(),
                OUTBOX_STATUS_PENDING,
            ));
            Ok(())
        }
    }

    #[test]
    fn webhook_publisher_rejects_unsupported_url_schemes() {
        let publisher = WebhookOutboxPublisher::new("ftp://events.example/hooks/iot");
        let event = AiotOutboxPendingEvent {
            tenant_id: 100001,
            organization_id: 0,
            event_id: "device_session:session-001:iot.device.session.started".to_string(),
            event_type: "iot.device.session.started".to_string(),
            event_version: "1".to_string(),
            aggregate_type: "device_session".to_string(),
            aggregate_id: "session-001".to_string(),
            payload_json: "{}".to_string(),
            trace_id: None,
            attempt_count: 0,
        };
        let error = publisher.publish(&event).expect_err("ftp must fail");
        assert!(error.contains("http://") || error.contains("https://"));
    }

    #[test]
    fn outbox_dispatcher_publishes_and_marks_events() {
        let repo = Arc::new(InMemoryOutboxRepo::default());
        repo.events.lock().unwrap().push(AiotOutboxPendingEvent {
            tenant_id: 100001,
            organization_id: 0,
            event_id: "device_session:session-001:iot.device.session.started".to_string(),
            event_type: "iot.device.session.started".to_string(),
            event_version: "1".to_string(),
            aggregate_type: "device_session".to_string(),
            aggregate_id: "session-001".to_string(),
            payload_json: "{}".to_string(),
            trace_id: None,
            attempt_count: 0,
        });
        let dispatcher = AiotOutboxDispatcher::new(
            repo.clone(),
            Arc::new(StructuredLogOutboxPublisher),
            AiotOutboxDispatcherConfig::default(),
        );
        assert_eq!(dispatcher.run_once(), 1);
        assert_eq!(repo.pending_lag_count(), 0);
        assert_eq!(repo.statuses.lock().unwrap().len(), 1);
    }
}
