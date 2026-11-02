use factory_application::workflows::autonomous_mission::WorkflowSupervisor;

#[tokio::test]
async fn test_workflow_supervisor_cancellation_propagation() {
    let supervisor = WorkflowSupervisor::new();
    let token1 = supervisor.token();
    let token2 = supervisor.token();

    assert!(!token1.is_cancelled());
    assert!(!token2.is_cancelled());

    let (tx1, rx1) = tokio::sync::oneshot::channel();
    let (tx2, rx2) = tokio::sync::oneshot::channel();

    // Spawn mock background monitor tasks
    tokio::spawn(async move {
        tokio::select! {
            _ = token1.cancelled() => {
                let _ = tx1.send("monitor1_stopped");
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {}
        }
    });

    tokio::spawn(async move {
        tokio::select! {
            _ = token2.cancelled() => {
                let _ = tx2.send("monitor2_stopped");
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {}
        }
    });

    // Trigger workflow cancellation
    supervisor.cancel();

    let res1 = tokio::time::timeout(std::time::Duration::from_millis(500), rx1).await;
    let res2 = tokio::time::timeout(std::time::Duration::from_millis(500), rx2).await;

    assert_eq!(res1.unwrap().unwrap(), "monitor1_stopped");
    assert_eq!(res2.unwrap().unwrap(), "monitor2_stopped");
}
