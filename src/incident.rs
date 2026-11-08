//! Incident tracking for alerting.

/// Incident status
#[derive(Debug, Clone, PartialEq)]
pub enum IncidentStatus {
    /// Open incident
    Open,
    /// Investigating incident
    Investigating,
    /// Mitigated incident
    Mitigated,
    /// Resolved incident
    Resolved,
}

/// Incident severity
#[derive(Debug, Clone, PartialEq)]
pub enum IncidentSeverity {
    /// Critical incident
    Critical,
    /// High incident
    High,
    /// Medium incident
    Medium,
    /// Low incident
    Low,
}

/// Incident
#[derive(Debug, Clone)]
pub struct Incident {
    /// Incident ID
    pub id: String,
    /// Incident title
    pub title: String,
    /// Incident description
    pub description: String,
    /// Incident severity
    pub severity: IncidentSeverity,
    /// Incident status
    pub status: IncidentStatus,
    /// Incident alerts
    pub alerts: Vec<String>,
    /// Incident assignee
    pub assignee: Option<String>,
    /// Incident labels
    pub labels: std::collections::HashMap<String, String>,
    /// Incident annotations
    pub annotations: std::collections::HashMap<String, String>,
    /// Incident starts at
    pub starts_at: u64,
    /// Incident ends at
    pub ends_at: Option<u64>,
    /// Incident created timestamp
    pub created_at: u64,
    /// Incident updated timestamp
    pub updated_at: u64,
    /// Incident comments
    pub comments: Vec<IncidentComment>,
    /// Incident timeline events
    pub timeline: Vec<TimelineEvent>,
}

/// Incident comment
#[derive(Debug, Clone)]
pub struct IncidentComment {
    /// Comment ID
    pub id: String,
    /// Comment author
    pub author: String,
    /// Comment content
    pub content: String,
    /// Comment created timestamp
    pub created_at: u64,
}

/// Timeline event
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    /// Event type
    pub event_type: TimelineEventType,
    /// Event description
    pub description: String,
    /// Event timestamp
    pub timestamp: u64,
    /// Event author
    pub author: Option<String>,
}

/// Timeline event type
#[derive(Debug, Clone, PartialEq)]
pub enum TimelineEventType {
    /// Created event
    Created,
    /// Updated event
    Updated,
    /// Status changed event
    StatusChanged,
    /// Assigned event
    Assigned,
    /// Commented event
    Commented,
    /// Resolved event
    Resolved,
}

impl Incident {
    /// Create new incident
    pub fn new(id: String, title: String, severity: IncidentSeverity) -> Self {
        Self {
            id,
            title,
            description: String::new(),
            severity,
            status: IncidentStatus::Open,
            alerts: vec![],
            assignee: None,
            labels: std::collections::HashMap::new(),
            annotations: std::collections::HashMap::new(),
            starts_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ends_at: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            comments: vec![],
            timeline: vec![TimelineEvent {
                event_type: TimelineEventType::Created,
                description: "Incident created".into(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                author: None,
            }],
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.add_timeline_event(TimelineEventType::Updated, "Description updated".to_string(), None);
    }
    
    /// Add alert
    pub fn add_alert(&mut self, alert_id: String) {
        if !self.alerts.contains(&alert_id) {
            self.alerts.push(alert_id);
            self.add_timeline_event(TimelineEventType::Updated, "Alert added".to_string(), None);
        }
    }
    
    /// Remove alert
    pub fn remove_alert(&mut self, alert_id: &str) -> bool {
        if let Some(pos) = self.alerts.iter().position(|a| a == alert_id) {
            self.alerts.remove(pos);
            self.add_timeline_event(TimelineEventType::Updated, "Alert removed".to_string(), None);
            return true;
        }
        false
    }
    
    /// Assign incident
    pub fn assign(&mut self, assignee: String) {
        self.assignee = Some(assignee.clone());
        self.add_timeline_event(TimelineEventType::Assigned, format!("Assigned to {}", assignee), None);
    }
    
    /// Unassign incident
    pub fn unassign(&mut self) {
        self.assignee = None;
        self.add_timeline_event(TimelineEventType::Updated, "Unassigned".to_string(), None);
    }
    
    /// Add label
    pub fn add_label(&mut self, key: String, value: String) {
        self.labels.insert(key, value);
        self.add_timeline_event(TimelineEventType::Updated, "Label added".to_string(), None);
    }
    
    /// Remove label
    pub fn remove_label(&mut self, key: &str) -> bool {
        if self.labels.remove(key).is_some() {
            self.add_timeline_event(TimelineEventType::Updated, "Label removed".to_string(), None);
            return true;
        }
        false
    }
    
    /// Add comment
    pub fn add_comment(&mut self, author: String, content: String) -> String {
        let comment_id = format!("comment-{}", self.comments.len() + 1);
        let comment = IncidentComment {
            id: comment_id.clone(),
            author,
            content,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.comments.push(comment);
        self.add_timeline_event(TimelineEventType::Commented, "Comment added".to_string(), None);
        comment_id
    }
    
    /// Set status
    pub fn set_status(&mut self, status: IncidentStatus) {
        let old_status = self.status.clone();
        let new_status = status.clone();
        self.status = status;
        
        match new_status {
            IncidentStatus::Resolved => {
                self.ends_at = Some(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs());
                self.add_timeline_event(TimelineEventType::Resolved, "Incident resolved".to_string(), None);
            }
            _ => {
                self.add_timeline_event(
                    TimelineEventType::StatusChanged,
                    format!("Status changed from {:?} to {:?}", old_status, new_status),
                    None,
                );
            }
        }
        
        self.update_timestamp();
    }
    
    /// Add timeline event
    pub fn add_timeline_event(&mut self, event_type: TimelineEventType, description: String, author: Option<String>) {
        let event = TimelineEvent {
            event_type,
            description,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            author,
        };
        self.timeline.push(event);
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if incident is open
    pub fn is_open(&self) -> bool {
        self.status == IncidentStatus::Open
    }
    
    /// Check if incident is investigating
    pub fn is_investigating(&self) -> bool {
        self.status == IncidentStatus::Investigating
    }
    
    /// Check if incident is mitigated
    pub fn is_mitigated(&self) -> bool {
        self.status == IncidentStatus::Mitigated
    }
    
    /// Check if incident is resolved
    pub fn is_resolved(&self) -> bool {
        self.status == IncidentStatus::Resolved
    }
    
    /// Check if incident is active (not resolved)
    pub fn is_active(&self) -> bool {
        self.status != IncidentStatus::Resolved
    }
    
    /// Get incident duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        let end = self.ends_at.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        end.saturating_sub(self.starts_at)
    }
    
    /// Get incident age in seconds
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
    
    /// Get alert count
    pub fn get_alert_count(&self) -> usize {
        self.alerts.len()
    }
    
    /// Get comment count
    pub fn get_comment_count(&self) -> usize {
        self.comments.len()
    }
    
    /// Get timeline event count
    pub fn get_timeline_count(&self) -> usize {
        self.timeline.len()
    }
    
    /// Check if incident has assignee
    pub fn has_assignee(&self) -> bool {
        self.assignee.is_some()
    }
    
    /// Check if incident has alerts
    pub fn has_alerts(&self) -> bool {
        !self.alerts.is_empty()
    }
    
    /// Check if incident has comments
    pub fn has_comments(&self) -> bool {
        !self.comments.is_empty()
    }
    
    /// Check if incident has labels
    pub fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }
    
    /// Get severity level (1-4, higher is more severe)
    pub fn get_severity_level(&self) -> u8 {
        match self.severity {
            IncidentSeverity::Critical => 4,
            IncidentSeverity::High => 3,
            IncidentSeverity::Medium => 2,
            IncidentSeverity::Low => 1,
        }
    }
}
