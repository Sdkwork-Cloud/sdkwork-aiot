#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceFields {
    pub trace_id: String,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub product_id: Option<String>,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
    pub connection_id: Option<String>,
    pub adapter_id: Option<String>,
    pub protocol_id: Option<String>,
    pub message_class: Option<String>,
    pub semantic_type: Option<String>,
    pub command_id: Option<String>,
}

impl TraceFields {
    pub fn new(trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            tenant_id: None,
            organization_id: None,
            product_id: None,
            device_id: None,
            session_id: None,
            connection_id: None,
            adapter_id: None,
            protocol_id: None,
            message_class: None,
            semantic_type: None,
            command_id: None,
        }
    }

    pub fn tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn device(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    pub fn protocol(mut self, protocol_id: impl Into<String>) -> Self {
        self.protocol_id = Some(protocol_id.into());
        self
    }

    pub fn session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }
}

pub fn redact_header(name: &str, value: &str) -> String {
    if is_sensitive_header(name) {
        "<redacted>".to_string()
    } else {
        value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeMetricFields {
    pub component: String,
    pub tenant_id: Option<String>,
    pub protocol_id: Option<String>,
    pub node_id: Option<String>,
    pub node_connections: Option<u64>,
    pub tenant_sessions: Option<u64>,
    pub device_inflight: Option<u64>,
    pub outbox_lag: Option<u64>,
    pub backpressure_action: Option<String>,
}

impl RuntimeMetricFields {
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            tenant_id: None,
            protocol_id: None,
            node_id: None,
            node_connections: None,
            tenant_sessions: None,
            device_inflight: None,
            outbox_lag: None,
            backpressure_action: None,
        }
    }

    pub fn tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn protocol(mut self, protocol_id: impl Into<String>) -> Self {
        self.protocol_id = Some(protocol_id.into());
        self
    }

    pub fn node(mut self, node_id: impl Into<String>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }

    pub fn connections(mut self, connections: u64) -> Self {
        self.node_connections = Some(connections);
        self
    }

    pub fn sessions(mut self, sessions: u64) -> Self {
        self.tenant_sessions = Some(sessions);
        self
    }

    pub fn device_inflight(mut self, inflight: u64) -> Self {
        self.device_inflight = Some(inflight);
        self
    }

    pub fn outbox_lag(mut self, lag: u64) -> Self {
        self.outbox_lag = Some(lag);
        self
    }

    pub fn backpressure(mut self, action: impl Into<String>) -> Self {
        self.backpressure_action = Some(action.into());
        self
    }

    pub fn contains_payload_fields(&self) -> bool {
        false
    }
}

fn is_sensitive_header(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "authorization" | "access-token" | "cookie" | "set-cookie" | "x-api-key"
    )
}
