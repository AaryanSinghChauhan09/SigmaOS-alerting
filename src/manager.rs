//! Alert manager for alerting.

use std::collections::HashMap;
use crate::alert::{Alert, AlertStatus, AlertSeverity};
use crate::rule::{AlertRule, RuleType, RuleStatus};
use crate::channel::{NotificationChannel, ChannelType, ChannelStatus};
use crate::incident::{Incident, IncidentStatus, IncidentSeverity};
use crate::escalation::{EscalationPolicy, EscalationStep};
use crate::monitor::{Monitor, MonitorType, MonitorStatus};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Auto-resolve alerts
    pub auto_resolve: bool,
    /// Auto-create incidents
    pub auto_create_incidents: bool,
    /// Max alerts per rule
    pub max_alerts_per_rule: usize,
    /// Alert retention period in seconds
    pub alert_retention: u64,
    /// Enable notifications
    pub enable_notifications: bool,
    /// Enable escalation
    pub enable_escalation: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            auto_resolve: true,
            auto_create_incidents: false,
            max_alerts_per_rule: 10,
            alert_retention: 86400,
            enable_notifications: true,
            enable_escalation: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total alerts
    pub total_alerts: usize,
    /// Total rules
    pub total_rules: usize,
    /// Total channels
    pub total_channels: usize,
    /// Total incidents
    pub total_incidents: usize,
    /// Total escalation policies
    pub total_escalation_policies: usize,
    /// Total monitors
    pub total_monitors: usize,
    /// Firing alerts
    pub firing_alerts: usize,
    /// Resolved alerts
    pub resolved_alerts: usize,
    /// Active incidents
    pub active_incidents: usize,
    /// Enabled rules
    pub enabled_rules: usize,
    /// Active channels
    pub active_channels: usize,
    /// Active monitors
    pub active_monitors: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Alert manager
#[derive(Debug, Clone)]
pub struct AlertManager {
    /// Alerts
    alerts: HashMap<String, Alert>,
    /// Rules
    rules: HashMap<String, AlertRule>,
    /// Channels
    channels: HashMap<String, NotificationChannel>,
    /// Incidents
    incidents: HashMap<String, Incident>,
    /// Escalation policies
    escalation_policies: HashMap<String, EscalationPolicy>,
    /// Monitors
    monitors: HashMap<String, Monitor>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AlertManager {
    fn default() -> Self {
        Self {
            alerts: HashMap::new(),
            rules: HashMap::new(),
            channels: HashMap::new(),
            incidents: HashMap::new(),
            escalation_policies: HashMap::new(),
            monitors: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_alerts: 0,
                total_rules: 0,
                total_channels: 0,
                total_incidents: 0,
                total_escalation_policies: 0,
                total_monitors: 0,
                firing_alerts: 0,
                resolved_alerts: 0,
                active_incidents: 0,
                enabled_rules: 0,
                active_channels: 0,
                active_monitors: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AlertManager {
    /// Create new alert manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            alerts: HashMap::new(),
            rules: HashMap::new(),
            channels: HashMap::new(),
            incidents: HashMap::new(),
            escalation_policies: HashMap::new(),
            monitors: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_alerts: 0,
                total_rules: 0,
                total_channels: 0,
                total_incidents: 0,
                total_escalation_policies: 0,
                total_monitors: 0,
                firing_alerts: 0,
                resolved_alerts: 0,
                active_incidents: 0,
                enabled_rules: 0,
                active_channels: 0,
                active_monitors: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add default notification channel
        let default_channel = NotificationChannel::new(
            "default".into(),
            "Default".into(),
            ChannelType::Syslog,
        );
        self.channels.insert("default".into(), default_channel);
        
        self.update_statistics();
    }
    
    // Alert methods
    
    /// Create alert
    pub fn create_alert(&mut self, title: String, severity: AlertSeverity) -> String {
        let id = format!("alert-{}", self.alerts.len() + 1);
        let alert = Alert::new(id.clone(), title, severity);
        
        self.alerts.insert(id.clone(), alert);
        self.update_statistics();
        id
    }
    
    /// Get alert
    pub fn get_alert(&self, alert_id: &str) -> Option<&Alert> {
        self.alerts.get(alert_id)
    }
    
    /// Get alert mutably
    pub fn get_alert_mut(&mut self, alert_id: &str) -> Option<&mut Alert> {
        self.alerts.get_mut(alert_id)
    }
    
    /// Get all alerts
    pub fn list_alerts(&self) -> Vec<&Alert> {
        self.alerts.values().collect()
    }
    
    /// Delete alert
    pub fn delete_alert(&mut self, alert_id: &str) -> bool {
        if self.alerts.remove(alert_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Resolve alert
    pub fn resolve_alert(&mut self, alert_id: &str) -> bool {
        if let Some(alert) = self.alerts.get_mut(alert_id) {
            alert.resolve();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Silence alert
    pub fn silence_alert(&mut self, alert_id: &str, duration_seconds: u64) -> bool {
        if let Some(alert) = self.alerts.get_mut(alert_id) {
            alert.silence(duration_seconds);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Unsilence alert
    pub fn unsilence_alert(&mut self, alert_id: &str) -> bool {
        if let Some(alert) = self.alerts.get_mut(alert_id) {
            alert.unsilence();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get alerts by status
    pub fn get_alerts_by_status(&self, status: &AlertStatus) -> Vec<&Alert> {
        self.alerts.values()
            .filter(|a| a.status == *status)
            .collect()
    }
    
    /// Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: &AlertSeverity) -> Vec<&Alert> {
        self.alerts.values()
            .filter(|a| a.severity == *severity)
            .collect()
    }
    
    /// Get firing alerts
    pub fn get_firing_alerts(&self) -> Vec<&Alert> {
        self.get_alerts_by_status(&AlertStatus::Firing)
    }
    
    /// Get resolved alerts
    pub fn get_resolved_alerts(&self) -> Vec<&Alert> {
        self.get_alerts_by_status(&AlertStatus::Resolved)
    }
    
    /// Get silenced alerts
    pub fn get_silenced_alerts(&self) -> Vec<&Alert> {
        self.get_alerts_by_status(&AlertStatus::Silenced)
    }
    
    /// Search alerts
    pub fn search_alerts(&self, query: &str) -> Vec<&Alert> {
        let query_lower = query.to_lowercase();
        self.alerts.values()
            .filter(|a| a.title.to_lowercase().contains(&query_lower) ||
                       a.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Rule methods
    
    /// Create rule
    pub fn create_rule(&mut self, name: String, rule_type: RuleType) -> String {
        let id = format!("rule-{}", self.rules.len() + 1);
        let rule = AlertRule::new(id.clone(), name, rule_type);
        
        self.rules.insert(id.clone(), rule);
        self.update_statistics();
        id
    }
    
    /// Get rule
    pub fn get_rule(&self, rule_id: &str) -> Option<&AlertRule> {
        self.rules.get(rule_id)
    }
    
    /// Get rule mutably
    pub fn get_rule_mut(&mut self, rule_id: &str) -> Option<&mut AlertRule> {
        self.rules.get_mut(rule_id)
    }
    
    /// Get all rules
    pub fn list_rules(&self) -> Vec<&AlertRule> {
        self.rules.values().collect()
    }
    
    /// Delete rule
    pub fn delete_rule(&mut self, rule_id: &str) -> bool {
        if self.rules.remove(rule_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Enable rule
    pub fn enable_rule(&mut self, rule_id: &str) -> bool {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disable rule
    pub fn disable_rule(&mut self, rule_id: &str) -> bool {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.disable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get rules by type
    pub fn get_rules_by_type(&self, rule_type: &RuleType) -> Vec<&AlertRule> {
        self.rules.values()
            .filter(|r| r.rule_type == *rule_type)
            .collect()
    }
    
    /// Get rules by status
    pub fn get_rules_by_status(&self, status: &RuleStatus) -> Vec<&AlertRule> {
        self.rules.values()
            .filter(|r| r.status == *status)
            .collect()
    }
    
    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&AlertRule> {
        self.rules.values()
            .filter(|r| r.is_enabled())
            .collect()
    }
    
    /// Search rules
    pub fn search_rules(&self, query: &str) -> Vec<&AlertRule> {
        let query_lower = query.to_lowercase();
        self.rules.values()
            .filter(|r| r.name.to_lowercase().contains(&query_lower) ||
                       r.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Channel methods
    
    /// Create channel
    pub fn create_channel(&mut self, name: String, channel_type: ChannelType) -> String {
        let id = format!("channel-{}", self.channels.len() + 1);
        let channel = NotificationChannel::new(id.clone(), name, channel_type);
        
        self.channels.insert(id.clone(), channel);
        self.update_statistics();
        id
    }
    
    /// Get channel
    pub fn get_channel(&self, channel_id: &str) -> Option<&NotificationChannel> {
        self.channels.get(channel_id)
    }
    
    /// Get channel mutably
    pub fn get_channel_mut(&mut self, channel_id: &str) -> Option<&mut NotificationChannel> {
        self.channels.get_mut(channel_id)
    }
    
    /// Get all channels
    pub fn list_channels(&self) -> Vec<&NotificationChannel> {
        self.channels.values().collect()
    }
    
    /// Delete channel
    pub fn delete_channel(&mut self, channel_id: &str) -> bool {
        if self.channels.remove(channel_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Enable channel
    pub fn enable_channel(&mut self, channel_id: &str) -> bool {
        if let Some(channel) = self.channels.get_mut(channel_id) {
            channel.enable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disable channel
    pub fn disable_channel(&mut self, channel_id: &str) -> bool {
        if let Some(channel) = self.channels.get_mut(channel_id) {
            channel.disable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get channels by type
    pub fn get_channels_by_type(&self, channel_type: &ChannelType) -> Vec<&NotificationChannel> {
        self.channels.values()
            .filter(|c| c.channel_type == *channel_type)
            .collect()
    }
    
    /// Get channels by status
    pub fn get_channels_by_status(&self, status: &ChannelStatus) -> Vec<&NotificationChannel> {
        self.channels.values()
            .filter(|c| c.status == *status)
            .collect()
    }
    
    /// Get active channels
    pub fn get_active_channels(&self) -> Vec<&NotificationChannel> {
        self.channels.values()
            .filter(|c| c.is_enabled())
            .collect()
    }
    
    // Incident methods
    
    /// Create incident
    pub fn create_incident(&mut self, title: String, severity: IncidentSeverity) -> String {
        let id = format!("incident-{}", self.incidents.len() + 1);
        let incident = Incident::new(id.clone(), title, severity);
        
        self.incidents.insert(id.clone(), incident);
        self.update_statistics();
        id
    }
    
    /// Get incident
    pub fn get_incident(&self, incident_id: &str) -> Option<&Incident> {
        self.incidents.get(incident_id)
    }
    
    /// Get incident mutably
    pub fn get_incident_mut(&mut self, incident_id: &str) -> Option<&mut Incident> {
        self.incidents.get_mut(incident_id)
    }
    
    /// Get all incidents
    pub fn list_incidents(&self) -> Vec<&Incident> {
        self.incidents.values().collect()
    }
    
    /// Delete incident
    pub fn delete_incident(&mut self, incident_id: &str) -> bool {
        if self.incidents.remove(incident_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Resolve incident
    pub fn resolve_incident(&mut self, incident_id: &str) -> bool {
        if let Some(incident) = self.incidents.get_mut(incident_id) {
            incident.set_status(IncidentStatus::Resolved);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get incidents by status
    pub fn get_incidents_by_status(&self, status: &IncidentStatus) -> Vec<&Incident> {
        self.incidents.values()
            .filter(|i| i.status == *status)
            .collect()
    }
    
    /// Get incidents by severity
    pub fn get_incidents_by_severity(&self, severity: &IncidentSeverity) -> Vec<&Incident> {
        self.incidents.values()
            .filter(|i| i.severity == *severity)
            .collect()
    }
    
    /// Get active incidents
    pub fn get_active_incidents(&self) -> Vec<&Incident> {
        self.incidents.values()
            .filter(|i| i.is_active())
            .collect()
    }
    
    /// Search incidents
    pub fn search_incidents(&self, query: &str) -> Vec<&Incident> {
        let query_lower = query.to_lowercase();
        self.incidents.values()
            .filter(|i| i.title.to_lowercase().contains(&query_lower) ||
                       i.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Escalation policy methods
    
    /// Create escalation policy
    pub fn create_escalation_policy(&mut self, name: String) -> String {
        let id = format!("escalation-{}", self.escalation_policies.len() + 1);
        let policy = EscalationPolicy::new(id.clone(), name);
        
        self.escalation_policies.insert(id.clone(), policy);
        self.update_statistics();
        id
    }
    
    /// Get escalation policy
    pub fn get_escalation_policy(&self, policy_id: &str) -> Option<&EscalationPolicy> {
        self.escalation_policies.get(policy_id)
    }
    
    /// Get escalation policy mutably
    pub fn get_escalation_policy_mut(&mut self, policy_id: &str) -> Option<&mut EscalationPolicy> {
        self.escalation_policies.get_mut(policy_id)
    }
    
    /// Get all escalation policies
    pub fn list_escalation_policies(&self) -> Vec<&EscalationPolicy> {
        self.escalation_policies.values().collect()
    }
    
    /// Delete escalation policy
    pub fn delete_escalation_policy(&mut self, policy_id: &str) -> bool {
        if self.escalation_policies.remove(policy_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    // Monitor methods
    
    /// Create monitor
    pub fn create_monitor(&mut self, name: String, monitor_type: MonitorType) -> String {
        let id = format!("monitor-{}", self.monitors.len() + 1);
        let monitor = Monitor::new(id.clone(), name, monitor_type);
        
        self.monitors.insert(id.clone(), monitor);
        self.update_statistics();
        id
    }
    
    /// Get monitor
    pub fn get_monitor(&self, monitor_id: &str) -> Option<&Monitor> {
        self.monitors.get(monitor_id)
    }
    
    /// Get monitor mutably
    pub fn get_monitor_mut(&mut self, monitor_id: &str) -> Option<&mut Monitor> {
        self.monitors.get_mut(monitor_id)
    }
    
    /// Get all monitors
    pub fn list_monitors(&self) -> Vec<&Monitor> {
        self.monitors.values().collect()
    }
    
    /// Delete monitor
    pub fn delete_monitor(&mut self, monitor_id: &str) -> bool {
        if self.monitors.remove(monitor_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Enable monitor
    pub fn enable_monitor(&mut self, monitor_id: &str) -> bool {
        if let Some(monitor) = self.monitors.get_mut(monitor_id) {
            monitor.enable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disable monitor
    pub fn disable_monitor(&mut self, monitor_id: &str) -> bool {
        if let Some(monitor) = self.monitors.get_mut(monitor_id) {
            monitor.disable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Record monitor check
    pub fn record_monitor_check(&mut self, monitor_id: &str) -> bool {
        if let Some(monitor) = self.monitors.get_mut(monitor_id) {
            monitor.record_check();
            return true;
        }
        false
    }
    
    /// Get monitors by type
    pub fn get_monitors_by_type(&self, monitor_type: &MonitorType) -> Vec<&Monitor> {
        self.monitors.values()
            .filter(|m| m.monitor_type == *monitor_type)
            .collect()
    }
    
    /// Get monitors by status
    pub fn get_monitors_by_status(&self, status: &MonitorStatus) -> Vec<&Monitor> {
        self.monitors.values()
            .filter(|m| m.status == *status)
            .collect()
    }
    
    /// Get active monitors
    pub fn get_active_monitors(&self) -> Vec<&Monitor> {
        self.monitors.values()
            .filter(|m| m.is_enabled())
            .collect()
    }
    
    /// Get due monitors
    pub fn get_due_monitors(&self) -> Vec<&Monitor> {
        self.monitors.values()
            .filter(|m| m.is_enabled() && m.is_due())
            .collect()
    }
    
    /// Search monitors
    pub fn search_monitors(&self, query: &str) -> Vec<&Monitor> {
        let query_lower = query.to_lowercase();
        self.monitors.values()
            .filter(|m| m.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_alerts = self.alerts.len();
        self.statistics.total_rules = self.rules.len();
        self.statistics.total_channels = self.channels.len();
        self.statistics.total_incidents = self.incidents.len();
        self.statistics.total_escalation_policies = self.escalation_policies.len();
        self.statistics.total_monitors = self.monitors.len();
        
        self.statistics.firing_alerts = self.alerts.values()
            .filter(|a| a.status == AlertStatus::Firing)
            .count();
        
        self.statistics.resolved_alerts = self.alerts.values()
            .filter(|a| a.status == AlertStatus::Resolved)
            .count();
        
        self.statistics.active_incidents = self.incidents.values()
            .filter(|i| i.is_active())
            .count();
        
        self.statistics.enabled_rules = self.rules.values()
            .filter(|r| r.is_enabled())
            .count();
        
        self.statistics.active_channels = self.channels.values()
            .filter(|c| c.is_enabled())
            .count();
        
        self.statistics.active_monitors = self.monitors.values()
            .filter(|m| m.is_enabled())
            .count();
        
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            alerts: self.alerts.values().cloned().collect(),
            rules: self.rules.values().cloned().collect(),
            channels: self.channels.values().cloned().collect(),
            incidents: self.incidents.values().cloned().collect(),
            escalation_policies: self.escalation_policies.values().cloned().collect(),
            monitors: self.monitors.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.alerts.clear();
        for alert in export.alerts {
            self.alerts.insert(alert.id.clone(), alert);
        }
        
        self.rules.clear();
        for rule in export.rules {
            self.rules.insert(rule.id.clone(), rule);
        }
        
        self.channels.clear();
        for channel in export.channels {
            self.channels.insert(channel.id.clone(), channel);
        }
        
        self.incidents.clear();
        for incident in export.incidents {
            self.incidents.insert(incident.id.clone(), incident);
        }
        
        self.escalation_policies.clear();
        for policy in export.escalation_policies {
            self.escalation_policies.insert(policy.id.clone(), policy);
        }
        
        self.monitors.clear();
        for monitor in export.monitors {
            self.monitors.insert(monitor.id.clone(), monitor);
        }
        
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Alerts
    pub alerts: Vec<Alert>,
    /// Rules
    pub rules: Vec<AlertRule>,
    /// Channels
    pub channels: Vec<NotificationChannel>,
    /// Incidents
    pub incidents: Vec<Incident>,
    /// Escalation policies
    pub escalation_policies: Vec<EscalationPolicy>,
    /// Monitors
    pub monitors: Vec<Monitor>,
}
