//! Notification channel definitions for alerting.

/// Channel type
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelType {
    /// Email channel
    Email,
    /// Slack channel
    Slack,
    /// Webhook channel
    Webhook,
    /// PagerDuty channel
    PagerDuty,
    /// SMS channel
    SMS,
    /// Syslog channel
    Syslog,
    /// Custom channel
    Custom,
}

/// Channel status
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelStatus {
    /// Active channel
    Active,
    /// Inactive channel
    Inactive,
    /// Error channel
    Error,
}

/// Notification channel
#[derive(Debug, Clone)]
pub struct NotificationChannel {
    /// Channel ID
    pub id: String,
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: ChannelType,
    /// Channel status
    pub status: ChannelStatus,
    /// Channel enabled
    pub enabled: bool,
    /// Channel configuration
    pub config: std::collections::HashMap<String, String>,
    /// Channel created timestamp
    pub created_at: u64,
    /// Channel updated timestamp
    pub updated_at: u64,
    /// Channel last used timestamp
    pub last_used: Option<u64>,
    /// Channel error count
    pub error_count: u64,
}

impl NotificationChannel {
    /// Create new notification channel
    pub fn new(id: String, name: String, channel_type: ChannelType) -> Self {
        Self {
            id,
            name,
            channel_type,
            status: ChannelStatus::Active,
            enabled: true,
            config: std::collections::HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_used: None,
            error_count: 0,
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
        self.update_timestamp();
    }
    
    /// Get config
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
    
    /// Remove config
    pub fn remove_config(&mut self, key: &str) -> bool {
        if self.config.remove(key).is_some() {
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Enable channel
    pub fn enable(&mut self) {
        self.enabled = true;
        self.status = ChannelStatus::Active;
        self.update_timestamp();
    }
    
    /// Disable channel
    pub fn disable(&mut self) {
        self.enabled = false;
        self.status = ChannelStatus::Inactive;
        self.update_timestamp();
    }
    
    /// Set error status
    pub fn set_error(&mut self) {
        self.status = ChannelStatus::Error;
        self.error_count += 1;
        self.update_timestamp();
    }
    
    /// Mark as used
    pub fn mark_used(&mut self) {
        self.last_used = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if channel is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled && self.status == ChannelStatus::Active
    }
    
    /// Check if channel is disabled
    pub fn is_disabled(&self) -> bool {
        !self.enabled || self.status == ChannelStatus::Inactive
    }
    
    /// Check if channel has error
    pub fn has_error(&self) -> bool {
        self.status == ChannelStatus::Error
    }
    
    /// Get channel age in seconds
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
    
    /// Get time since last use in seconds
    pub fn get_time_since_last_use(&self) -> Option<u64> {
        self.last_used.map(|last| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - last
        })
    }
    
    /// Get config count
    pub fn get_config_count(&self) -> usize {
        self.config.len()
    }
    
    /// Check if channel has config
    pub fn has_config(&self) -> bool {
        !self.config.is_empty()
    }
    
    /// Get error rate
    pub fn get_error_rate(&self) -> f64 {
        let age_hours = self.get_age_seconds() as f64 / 3600.0;
        if age_hours == 0.0 {
            return 0.0;
        }
        self.error_count as f64 / age_hours
    }
}
