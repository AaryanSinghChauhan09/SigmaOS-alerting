use sigmaos_component::{AlertManager, AlertSeverity, RuleType, ChannelType, MonitorType};

#[test]
fn test_manager_lifecycle() {
    let mut manager = AlertManager::new();
    manager.initialize();

    let alert_id = manager.create_alert("CPU high".into(), AlertSeverity::High);
    let rule_id = manager.create_rule("cpu_rule".into(), RuleType::Threshold);
    let channel_id = manager.create_channel("slack".into(), ChannelType::Slack);
    let monitor_id = manager.create_monitor("cpu_monitor".into(), MonitorType::CPU);

    assert!(manager.get_alert(&alert_id).is_some());
    assert!(manager.get_rule(&rule_id).is_some());
    assert!(manager.get_channel(&channel_id).is_some());
    assert!(manager.get_monitor(&monitor_id).is_some());

    let stats = manager.get_statistics();
    assert_eq!(stats.total_alerts, 1);
    assert_eq!(stats.total_rules, 1);
    assert_eq!(stats.total_channels, 2); // includes default
    assert_eq!(stats.total_monitors, 1);
}

#[test]
fn test_alert_workflow() {
    let mut manager = AlertManager::new();
    let alert_id = manager.create_alert("Disk full".into(), AlertSeverity::Critical);

    assert!(manager.resolve_alert(&alert_id));
    assert!(manager.get_alert(&alert_id).unwrap().is_resolved());

    let alert_id2 = manager.create_alert("Mem high".into(), AlertSeverity::Medium);
    assert!(manager.silence_alert(&alert_id2, 3600));
    assert!(manager.get_alert(&alert_id2).unwrap().is_silenced());
}

#[test]
fn test_export_import_roundtrip() {
    let mut manager = AlertManager::new();
    manager.create_alert("A1".into(), AlertSeverity::Low);
    manager.create_rule("R1".into(), RuleType::Threshold);

    let export = manager.export_data();
    let mut manager2 = AlertManager::new();
    manager2.import_data(export);

    assert_eq!(manager2.list_alerts().len(), 1);
    assert_eq!(manager2.list_rules().len(), 1);
}
