//! Monitor definitions for alerting.

/// Monitor type
#[derive(Debug, Clone, PartialEq)]
pub enum MonitorType {
    /// CPU monitor
    CPU,
    /// Memory monitor
    Memory,
    /// Disk monitor
    Disk,
    /// Network monitor
    Network,
    /// Service monitor
    Service,
    /// Custom monitor
    Custom,
}

/// Monitor status
#[derive(Debug, Clone, PartialEq)]
pub enum MonitorStatus {
    /// Active monitor
    Active,
    /// Inactive monitor
    Inactive,
    /// Error monitor
    Error,
}

/// Monitor target
#[derive(Debug, Clone, PartialEq)]
pub enum MonitorTarget {
    /// Local target
    Local,
    /// Remote target
    Remote(String),
    /// Custom target
    Custom(String),
}

/// Monitor
#[derive(Debug, Clone)]
pub struct Monitor {
    /// Monitor ID
    pub id: String,
    /// Monitor name
    pub name: String,
    /// Monitor type
    pub monitor_type: MonitorType,
    /// Monitor status
    pub status: MonitorStatus,
    /// Monitor target
    pub target: MonitorTarget,
    /// Monitor enabled
    pub enabled: bool,
    /// Monitor interval in seconds
    pub interval: u64,
    /// Monitor timeout in seconds
    pub timeout: u64,
    /// Monitor configuration
    pub config: std::collections::HashMap<String, String>,
    /// Monitor created timestamp
    pub created_at: u64,
    /// Monitor updated timestamp
    pub updated_at: u64,
    /// Monitor last check timestamp
    pub last_check: Option<u64>,
    /// Monitor check count
    pub check_count: u64,
    /// Monitor error count
    pub error_count: u64,
}

impl Monitor {
    /// Create new monitor
    pub fn new(id: String, name: String, monitor_type: MonitorType) -> Self {
        Self {
            id,
            name,
            monitor_type,
            status: MonitorStatus::Active,
            target: MonitorTarget::Local,
            enabled: true,
            interval: 60,
            timeout: 30,
            config: std::collections::HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_check: None,
            check_count: 0,
            error_count: 0,
        }
    }
    
    /// Set target
    pub fn set_target(&mut self, target: MonitorTarget) {
        self.target = target;
        self.update_timestamp();
    }
    
    /// Set interval
    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
        self.update_timestamp();
    }
    
    /// Set timeout
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
        self.update_timestamp();
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
    
    /// Enable monitor
    pub fn enable(&mut self) {
        self.enabled = true;
        self.status = MonitorStatus::Active;
        self.update_timestamp();
    }
    
    /// Disable monitor
    pub fn disable(&mut self) {
        self.enabled = false;
        self.status = MonitorStatus::Inactive;
        self.update_timestamp();
    }
    
    /// Set error status
    pub fn set_error(&mut self) {
        self.status = MonitorStatus::Error;
        self.error_count += 1;
        self.update_timestamp();
    }
    
    /// Record check
    pub fn record_check(&mut self) {
        self.last_check = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.check_count += 1;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if monitor is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled && self.status == MonitorStatus::Active
    }
    
    /// Check if monitor is disabled
    pub fn is_disabled(&self) -> bool {
        !self.enabled || self.status == MonitorStatus::Inactive
    }
    
    /// Check if monitor has error
    pub fn has_error(&self) -> bool {
        self.status == MonitorStatus::Error
    }
    
    /// Check if monitor is due for check
    pub fn is_due(&self) -> bool {
        if let Some(last_check) = self.last_check {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now - last_check >= self.interval
        } else {
            true
        }
    }
    
    /// Get monitor age in seconds
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
    
    /// Get time since last check in seconds
    pub fn get_time_since_last_check(&self) -> Option<u64> {
        self.last_check.map(|last| {
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
    
    /// Check if monitor has config
    pub fn has_config(&self) -> bool {
        !self.config.is_empty()
    }
    
    /// Get error rate
    pub fn get_error_rate(&self) -> f64 {
        if self.check_count == 0 {
            return 0.0;
        }
        self.error_count as f64 / self.check_count as f64 * 100.0
    }
}
