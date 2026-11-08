//! Escalation policy definitions for alerting.

/// Escalation trigger
#[derive(Debug, Clone, PartialEq)]
pub enum EscalationTrigger {
    /// Time based trigger
    Time(u64),
    /// Severity based trigger
    Severity(super::alert::AlertSeverity),
    /// Acknowledgment timeout trigger
    AckTimeout(u64),
    /// Custom trigger
    Custom(String),
}

/// Escalation step
#[derive(Debug, Clone)]
pub struct EscalationStep {
    /// Step ID
    pub id: String,
    /// Step name
    pub name: String,
    /// Step trigger
    pub trigger: EscalationTrigger,
    /// Step channels
    pub channels: Vec<String>,
    /// Step enabled
    pub enabled: bool,
    /// Step created timestamp
    pub created_at: u64,
}

impl EscalationStep {
    /// Create new escalation step
    pub fn new(id: String, name: String, trigger: EscalationTrigger) -> Self {
        Self {
            id,
            name,
            trigger,
            channels: vec![],
            enabled: true,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add channel
    pub fn add_channel(&mut self, channel_id: String) {
        if !self.channels.contains(&channel_id) {
            self.channels.push(channel_id);
        }
    }
    
    /// Remove channel
    pub fn remove_channel(&mut self, channel_id: &str) -> bool {
        if let Some(pos) = self.channels.iter().position(|c| c == channel_id) {
            self.channels.remove(pos);
            return true;
        }
        false
    }
    
    /// Enable step
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable step
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Check if step is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Escalation policy
#[derive(Debug, Clone)]
pub struct EscalationPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Policy steps
    pub steps: Vec<EscalationStep>,
    /// Policy enabled
    pub enabled: bool,
    /// Policy repeat escalation
    pub repeat_escalation: bool,
    /// Policy max escalations
    pub max_escalations: u32,
    /// Policy created timestamp
    pub created_at: u64,
    /// Policy updated timestamp
    pub updated_at: u64,
}

impl EscalationPolicy {
    /// Create new escalation policy
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            steps: vec![],
            enabled: true,
            repeat_escalation: false,
            max_escalations: 3,
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
    
    /// Add step
    pub fn add_step(&mut self, step: EscalationStep) -> String {
        let id = step.id.clone();
        self.steps.push(step);
        self.update_timestamp();
        id
    }
    
    /// Remove step
    pub fn remove_step(&mut self, step_id: &str) -> bool {
        if let Some(pos) = self.steps.iter().position(|s| s.id == step_id) {
            self.steps.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Get step
    pub fn get_step(&self, step_id: &str) -> Option<&EscalationStep> {
        self.steps.iter().find(|s| s.id == step_id)
    }
    
    /// Get step mutably
    pub fn get_step_mut(&mut self, step_id: &str) -> Option<&mut EscalationStep> {
        self.steps.iter_mut().find(|s| s.id == step_id)
    }
    
    /// Get all steps
    pub fn get_steps(&self) -> &[EscalationStep] {
        &self.steps
    }
    
    /// Enable policy
    pub fn enable(&mut self) {
        self.enabled = true;
        self.update_timestamp();
    }
    
    /// Disable policy
    pub fn disable(&mut self) {
        self.enabled = false;
        self.update_timestamp();
    }
    
    /// Set repeat escalation
    pub fn set_repeat_escalation(&mut self, repeat: bool) {
        self.repeat_escalation = repeat;
        self.update_timestamp();
    }
    
    /// Set max escalations
    pub fn set_max_escalations(&mut self, max: u32) {
        self.max_escalations = max;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if policy is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Check if policy is disabled
    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }
    
    /// Get step count
    pub fn get_step_count(&self) -> usize {
        self.steps.len()
    }
    
    /// Get enabled step count
    pub fn get_enabled_step_count(&self) -> usize {
        self.steps.iter().filter(|s| s.is_enabled()).count()
    }
    
    /// Check if policy has steps
    pub fn has_steps(&self) -> bool {
        !self.steps.is_empty()
    }
    
    /// Check if policy has enabled steps
    pub fn has_enabled_steps(&self) -> bool {
        self.steps.iter().any(|s| s.is_enabled())
    }
    
    /// Get policy age in seconds
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
}
