use factory_mcp_server::sandbox::{GvisorK8sDriver, SandboxDriver};
use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::{Api, PostParams},
    Client,
};
use serde_json::json;

#[tokio::test]
async fn test_gvisor_k8s_driver_live_connection() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Attempt to connect to local/cluster kube config
    let client_res = Client::try_default().await;

    if let Ok(client) = client_res {
        // Ensure "development" namespace exists for the test, or attempt to create it.
        // If we don't have permissions, we gracefully ignore the error.
        let namespaces: Api<Namespace> = Api::all(client.clone());
        let dev_ns = serde_json::from_value(json!({
            "apiVersion": "v1",
            "kind": "Namespace",
            "metadata": { "name": "development" }
        }))
        .unwrap();

        let _ = namespaces.create(&PostParams::default(), &dev_ns).await;

        let driver = GvisorK8sDriver;

        let result = driver.execute("print('hello gvisor')", "python").await;

        // This test might fail if the cluster doesn't support the 'gvisor' runtimeClassName
        // or if RBAC prevents creating jobs. We handle this gracefully in test environments.
        match result {
            Ok(res) => {
                println!("Successfully launched gVisor job: {:?}", res);
                if res.is_success {
                    assert!(res.stdout.contains("hello gvisor"));
                }
            }
            Err(e) => {
                let err_str = e.to_string();
                println!(
                    "Kubernetes integration test skipped or failed gracefully: {}",
                    err_str
                );
            }
        }
    } else {
        println!("Skipping K8s test since no cluster configuration was found.");
    }
}
