use factory_cli::operator::metrics::OperatorMetrics;

#[test]
fn test_status_patch_write_ratio_under_limit() {
    let metrics = OperatorMetrics::new();

    // Simulate 5 full mission lifecycles
    for _ in 0..5 {
        metrics.inc_active_reconciliation();
        // Phase transitions: Pending -> Initializing -> Running -> Completed
        metrics.record_status_patch(); // Initializing
        metrics.record_status_patch(); // Running
        metrics.record_status_patch(); // Completed
        metrics.record_completed_mission();
        metrics.dec_active_reconciliation();
    }

    assert_eq!(metrics.completed_missions_total(), 5);
    assert_eq!(metrics.status_patches_total(), 15);
    let ratio = metrics.patch_write_ratio();
    println!("Status patch write ratio: {:.2}", ratio);

    // Verify ratio is strictly <= 6 patches per mission
    assert!(
        ratio <= 6.0,
        "Patch write ratio exceeds KPI threshold: {:.2} > 6.0",
        ratio
    );

    let prom_text = metrics.render_prometheus();
    assert!(prom_text.contains("factory_operator_patch_write_ratio 3.00"));
}

