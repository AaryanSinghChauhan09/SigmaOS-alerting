//! Alert rule definitions for alerting.

/// Rule type
#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    /// Threshold rule
    Threshold,
    /// Change rule
    Change,
    /// Absence rule
    Absence,
    /// Anomaly rule
    Anomaly,
    /// Composite rule,
    Composite,
}

/// Rule status
#[derive(Debug, Clone, PartialEq)]
pub enum RuleStatus {
    /// Enabled rule
    Enabled,
    /// Disabled rule
    Disabled,
    /// Error rule
    Error,
}

/// Rule condition
#[derive(Debug, Clone, PartialEq)]
pub enum RuleCondition {
    /// Greater than
    GreaterThan(f64),
    /// Less than
    LessThan(f64),
    /// Equal to
    EqualTo(f64),
    /// Not equal to
    NotEqualTo(f64),
    /// Greater than or equal
    GreaterThanOrEqual(f64),
    /// Less than or equal
    LessThanOrEqual(f64),
}

/// Alert rule
#[derive(Debug, Clone)]
pub struct AlertRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule type
    pub rule_type: RuleType,
    /// Rule status
    pub status: RuleStatus,
    /// Rule enabled
    pub enabled: bool,
    /// Rule condition
    pub condition: Option<RuleCondition>,
    /// Rule query
    pub query: String,
    /// Rule severity
    pub severity: super::alert::AlertSeverity,
    /// Rule for duration in seconds
    pub for_duration: u64,
    /// Rule labels
    pub labels: std::collections::HashMap<String, String>,
    /// Rule annotations
    pub annotations: std::collections::HashMap<String, String>,
    /// Rule created timestamp
    pub created_at: u64,
    /// Rule updated timestamp
    pub updated_at: u64,
}

impl AlertRule {
    /// Create new alert rule
    pub fn new(id: String, name: String, rule_type: RuleType) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            rule_type,
            status: RuleStatus::Enabled,
            enabled: true,
            condition: None,
            query: String::new(),
            severity: super::alert::AlertSeverity::Medium,
            for_duration: 0,
            labels: std::collections::HashMap::new(),
            annotations: std::collections::HashMap::new(),
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
    
    /// Set condition
    pub fn set_condition(&mut self, condition: RuleCondition) {
        self.condition = Some(condition);
        self.update_timestamp();
    }
    
    /// Clear condition
    pub fn clear_condition(&mut self) {
        self.condition = None;
        self.update_timestamp();
    }
    
    /// Set query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.update_timestamp();
    }
    
    /// Set severity
    pub fn set_severity(&mut self, severity: super::alert::AlertSeverity) {
        self.severity = severity;
        self.update_timestamp();
    }
    
    /// Set for duration
    pub fn set_for_duration(&mut self, duration: u64) {
        self.for_duration = duration;
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
    
    /// Enable rule
    pub fn enable(&mut self) {
        self.enabled = true;
        self.status = RuleStatus::Enabled;
        self.update_timestamp();
    }
    
    /// Disable rule
    pub fn disable(&mut self) {
        self.enabled = false;
        self.status = RuleStatus::Disabled;
        self.update_timestamp();
    }
    
    /// Set error status
    pub fn set_error(&mut self) {
        self.status = RuleStatus::Error;
        self.enabled = false;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if rule is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled && self.status == RuleStatus::Enabled
    }
    
    /// Check if rule is disabled
    pub fn is_disabled(&self) -> bool {
        !self.enabled || self.status == RuleStatus::Disabled
    }
    
    /// Check if rule has error
    pub fn has_error(&self) -> bool {
        self.status == RuleStatus::Error
    }
    
    /// Get rule age in seconds
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
    
    /// Check if rule has condition
    pub fn has_condition(&self) -> bool {
        self.condition.is_some()
    }
    
    /// Check if rule has query
    pub fn has_query(&self) -> bool {
        !self.query.is_empty()
    }
    
    /// Check if rule has labels
    pub fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }
    
    /// Check if rule has annotations
    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }
    
    /// Check if rule has for duration
    pub fn has_for_duration(&self) -> bool {
        self.for_duration > 0
    }
}
