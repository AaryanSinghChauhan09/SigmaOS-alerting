//! Alert definitions for alerting.

/// Alert status
#[derive(Debug, Clone, PartialEq)]
pub enum AlertStatus {
    /// Firing alert
    Firing,
    /// Resolved alert
    Resolved,
    /// Silenced alert
    Silenced,
    /// Pending alert
    Pending,
}

/// Alert severity
#[derive(Debug, Clone, PartialEq)]
pub enum AlertSeverity {
    /// Critical severity
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
    /// Info severity
    Info,
}

/// Alert source
#[derive(Debug, Clone, PartialEq)]
pub enum AlertSource {
    /// Prometheus source
    Prometheus,
    /// Grafana source
    Grafana,
    /// Custom source
    Custom,
    /// Manual source
    Manual,
}

/// Alert
#[derive(Debug, Clone)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    /// Alert title
    pub title: String,
    /// Alert description
    pub description: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert status
    pub status: AlertStatus,
    /// Alert source
    pub source: AlertSource,
    /// Alert rule ID
    pub rule_id: Option<String>,
    /// Alert labels
    pub labels: std::collections::HashMap<String, String>,
    /// Alert annotations
    pub annotations: std::collections::HashMap<String, String>,
    /// Alert starts at
    pub starts_at: u64,
    /// Alert ends at
    pub ends_at: Option<u64>,
    /// Alert silenced until
    pub silenced_until: Option<u64>,
    /// Alert created timestamp
    pub created_at: u64,
    /// Alert updated timestamp
    pub updated_at: u64,
}

impl Alert {
    /// Create new alert
    pub fn new(id: String, title: String, severity: AlertSeverity) -> Self {
        Self {
            id,
            title,
            description: String::new(),
            severity,
            status: AlertStatus::Firing,
            source: AlertSource::Manual,
            rule_id: None,
            labels: std::collections::HashMap::new(),
            annotations: std::collections::HashMap::new(),
            starts_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ends_at: None,
            silenced_until: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set source
    pub fn set_source(&mut self, source: AlertSource) {
        self.source = source;
        self.update_timestamp();
    }
    
    /// Set rule ID
    pub fn set_rule_id(&mut self, rule_id: String) {
        self.rule_id = Some(rule_id);
        self.update_timestamp();
    }
    
    /// Add label
    pub fn add_label(&mut self, key: String, value: String) {
        self.labels.insert(key, value);
        self.update_timestamp();
    }
    
    /// Remove label
    pub fn remove_label(&mut self, key: &str) -> bool {
        if self.labels.remove(key).is_some() {
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Get label
    pub fn get_label(&self, key: &str) -> Option<&String> {
        self.labels.get(key)
    }
    
    /// Add annotation
    pub fn add_annotation(&mut self, key: String, value: String) {
        self.annotations.insert(key, value);
        self.update_timestamp();
    }
    
    /// Remove annotation
    pub fn remove_annotation(&mut self, key: &str) -> bool {
        if self.annotations.remove(key).is_some() {
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set ends at
    pub fn set_ends_at(&mut self, ends_at: u64) {
        self.ends_at = Some(ends_at);
        self.update_timestamp();
    }
    
    /// Set silenced until
    pub fn set_silenced_until(&mut self, silenced_until: u64) {
        self.silenced_until = Some(silenced_until);
        self.update_timestamp();
    }
    
    /// Resolve alert
    pub fn resolve(&mut self) {
        self.status = AlertStatus::Resolved;
        self.ends_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.update_timestamp();
    }
    
    /// Silence alert
    pub fn silence(&mut self, duration_seconds: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.silenced_until = Some(now + duration_seconds);
        self.status = AlertStatus::Silenced;
        self.update_timestamp();
    }
    
    /// Unsilence alert
    pub fn unsilence(&mut self) {
        self.silenced_until = None;
        if self.status == AlertStatus::Silenced {
            self.status = AlertStatus::Firing;
        }
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if alert is firing
    pub fn is_firing(&self) -> bool {
        self.status == AlertStatus::Firing
    }
    
    /// Check if alert is resolved
    pub fn is_resolved(&self) -> bool {
        self.status == AlertStatus::Resolved
    }
    
    /// Check if alert is silenced
    pub fn is_silenced(&self) -> bool {
        self.status == AlertStatus::Silenced
    }
    
    /// Check if alert is pending
    pub fn is_pending(&self) -> bool {
        self.status == AlertStatus::Pending
    }
    
    /// Check if alert is active (firing or pending)
    pub fn is_active(&self) -> bool {
        matches!(self.status, AlertStatus::Firing | AlertStatus::Pending)
    }
    
    /// Check if alert is silenced now
    pub fn is_silenced_now(&self) -> bool {
        if let Some(silenced_until) = self.silenced_until {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now < silenced_until
        } else {
            false
        }
    }
    
    /// Get alert duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        let end = self.ends_at.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        end.saturating_sub(self.starts_at)
    }
    
    /// Get alert age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.updated_at
    }
    
    /// Get label count
    pub fn get_label_count(&self) -> usize {
        self.labels.len()
    }
    
    /// Get annotation count
    pub fn get_annotation_count(&self) -> usize {
        self.annotations.len()
    }
    
    /// Check if alert has labels
    pub fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }
    
    /// Check if alert has annotations
    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    
    /// Check if alert has rule
    pub fn has_rule(&self) -> bool {
        self.rule_id.is_some()
    }
    
    /// Get severity level (1-5, higher is more severe)
    pub fn get_severity_level(&self) -> u8 {
        match self.severity {
            AlertSeverity::Critical => 5,
            AlertSeverity::High => 4,
            AlertSeverity::Medium => 3,
            AlertSeverity::Low => 2,
            AlertSeverity::Info => 1,
        }
    }
}
