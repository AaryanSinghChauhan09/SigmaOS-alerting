//! SigmaOS-alerting — comprehensive monitoring and alerting component.
//!
//! Provides alert rule management, notification channels, incident tracking,
//! escalation policies, and monitoring dashboards for system observability.

pub mod alert;
pub mod rule;
pub mod channel;
pub mod incident;
pub mod escalation;
pub mod monitor;
pub mod manager;
pub mod options;

pub use alert::{Alert, AlertStatus, AlertSeverity, AlertSource};
pub use rule::{AlertRule, RuleCondition, RuleType, RuleStatus};
pub use channel::{NotificationChannel, ChannelType, ChannelStatus};
pub use incident::{Incident, IncidentStatus, IncidentSeverity};
pub use escalation::{EscalationPolicy, EscalationStep, EscalationTrigger};
pub use monitor::{Monitor, MonitorType, MonitorStatus, MonitorTarget};
pub use manager::{AlertManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alert_status() {
        let statuses = vec![
            AlertStatus::Firing,
            AlertStatus::Resolved,
            AlertStatus::Silenced,
            AlertStatus::Pending,
        ];
        assert_eq!(statuses.len(), 4);
    }
    
    #[test]
    fn test_alert_severity() {
        let severities = vec![
            AlertSeverity::Critical,
            AlertSeverity::High,
            AlertSeverity::Medium,
            AlertSeverity::Low,
            AlertSeverity::Info,
        ];
        assert_eq!(severities.len(), 5);
    }
    
    #[test]
    fn test_rule_type() {
        let types = vec![
            RuleType::Threshold,
            RuleType::Change,
            RuleType::Absence,
            RuleType::Anomaly,
            RuleType::Composite,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_channel_type() {
        let types = vec![
            ChannelType::Email,
            ChannelType::Slack,
            ChannelType::Webhook,
            ChannelType::PagerDuty,
            ChannelType::SMS,
            ChannelType::Syslog,
            ChannelType::Custom,
        ];
        assert_eq!(types.len(), 7);
    }
    
    #[test]
    fn test_incident_status() {
        let statuses = vec![
            IncidentStatus::Open,
            IncidentStatus::Investigating,
            IncidentStatus::Mitigated,
            IncidentStatus::Resolved,
        ];
        assert_eq!(statuses.len(), 4);
    }
    
    #[test]
    fn test_monitor_type() {
        let types = vec![
            MonitorType::CPU,
            MonitorType::Memory,
            MonitorType::Disk,
            MonitorType::Network,
            MonitorType::Service,
            MonitorType::Custom,
        ];
        assert_eq!(types.len(), 6);
    }
    
    #[test]
    fn test_alert_creation() {
        let alert = Alert::new(
            "alert-1".into(),
            "High CPU Usage".into(),
            AlertSeverity::High,
        );
        
        assert_eq!(alert.id, "alert-1");
        assert_eq!(alert.title, "High CPU Usage");
        assert_eq!(alert.severity, AlertSeverity::High);
        assert_eq!(alert.status, AlertStatus::Firing);
    }
    
    #[test]
    fn test_alert_rule_creation() {
        let rule = AlertRule::new(
            "rule-1".into(),
            "CPU Usage > 90%".into(),
            RuleType::Threshold,
        );
        
        assert_eq!(rule.id, "rule-1");
        assert_eq!(rule.name, "CPU Usage > 90%");
        assert_eq!(rule.rule_type, RuleType::Threshold);
    }
    
    #[test]
    fn test_notification_channel_creation() {
        let channel = NotificationChannel::new(
            "channel-1".into(),
            "Ops Team Slack".into(),
            ChannelType::Slack,
        );
        
        assert_eq!(channel.id, "channel-1");
        assert_eq!(channel.name, "Ops Team Slack");
        assert_eq!(channel.channel_type, ChannelType::Slack);
    }
    
    #[test]
    fn test_incident_creation() {
        let incident = Incident::new(
            "incident-1".into(),
            "Database Outage".into(),
            IncidentSeverity::Critical,
        );
        
        assert_eq!(incident.id, "incident-1");
        assert_eq!(incident.title, "Database Outage");
        assert_eq!(incident.severity, IncidentSeverity::Critical);
    }
    
    #[test]
    fn test_monitor_creation() {
        let monitor = Monitor::new(
            "monitor-1".into(),
            "Web Server CPU".into(),
            MonitorType::CPU,
        );
        
        assert_eq!(monitor.id, "monitor-1");
        assert_eq!(monitor.name, "Web Server CPU");
        assert_eq!(monitor.monitor_type, MonitorType::CPU);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AlertManager::new();
        assert!(manager.list_alerts().is_empty());
        assert!(manager.list_rules().is_empty());
    }
    
    #[test]
    fn test_create_alert() {
        let mut manager = AlertManager::new();
        let alert_id = manager.create_alert(
            "High CPU".into(),
            AlertSeverity::High,
        );
        
        assert!(manager.get_alert(&alert_id).is_some());
        assert_eq!(manager.list_alerts().len(), 1);
    }
    
    #[test]
    fn test_resolve_alert() {
        let mut manager = AlertManager::new();
        let alert_id = manager.create_alert("Test".into(), AlertSeverity::Low);
        
        assert!(manager.resolve_alert(&alert_id));
        assert_eq!(manager.get_alert(&alert_id).unwrap().status, AlertStatus::Resolved);
    }
    
    #[test]
    fn test_silence_alert() {
        let mut manager = AlertManager::new();
        let alert_id = manager.create_alert("Test".into(), AlertSeverity::Medium);
        
        assert!(manager.silence_alert(&alert_id, 3600));
        assert_eq!(manager.get_alert(&alert_id).unwrap().status, AlertStatus::Silenced);
    }
    
    #[test]
    fn test_create_rule() {
        let mut manager = AlertManager::new();
        let rule_id = manager.create_rule(
            "CPU > 90%".into(),
            RuleType::Threshold,
        );
        
        assert!(manager.get_rule(&rule_id).is_some());
        assert_eq!(manager.list_rules().len(), 1);
    }
    
    #[test]
    fn test_enable_disable_rule() {
        let mut manager = AlertManager::new();
        let rule_id = manager.create_rule("Test".into(), RuleType::Threshold);
        
        assert!(manager.disable_rule(&rule_id));
        assert!(!manager.get_rule(&rule_id).unwrap().enabled);
        
        assert!(manager.enable_rule(&rule_id));
        assert!(manager.get_rule(&rule_id).unwrap().enabled);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AlertManager::new();
        
        manager.create_alert("Alert1".into(), AlertSeverity::High);
        manager.create_rule("Rule1".into(), RuleType::Threshold);
        
        let export = manager.export_data();
        
        let mut manager2 = AlertManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_alerts().len(), 1);
        assert_eq!(manager2.list_rules().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AlertManager::new();
        
        manager.create_alert("Alert1".into(), AlertSeverity::Critical);
        manager.create_alert("Alert2".into(), AlertSeverity::Low);
        manager.create_rule("Rule1".into(), RuleType::Threshold);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_alerts, 2);
        assert_eq!(stats.total_rules, 1);
        assert_eq!(stats.firing_alerts, 2);
    }
}
