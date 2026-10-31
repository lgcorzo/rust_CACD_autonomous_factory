pub mod crd;
pub mod admission_test;

use anyhow::Result;
use k8s_openapi::api::coordination::v1::Lease;
use kube::{
    api::{Api, Patch, PatchParams},
    Client,
};
use std::time::Duration;
use tracing::{error, info, warn};

/// Operator configuration options
#[derive(Clone, Debug)]
pub struct OperatorConfig {
    pub namespace: String,
    pub leader_election: bool,
    pub lease_name: String,
    pub lease_duration: Duration,
    pub renew_deadline: Duration,
}

impl Default for OperatorConfig {
    fn default() -> Self {
        Self {
            namespace: "agents".to_string(),
            leader_election: true,
            lease_name: "darkgravity-operator-lease".to_string(),
            lease_duration: Duration::from_secs(15),
            renew_deadline: Duration::from_secs(10),
        }
    }
}

/// Run the Kubernetes Operator loop with leader election
pub async fn run_operator(config: OperatorConfig) -> Result<()> {
    info!(
        namespace = %config.namespace,
        leader_election = config.leader_election,
        lease_name = %config.lease_name,
        "Initializing Dark Gravity AgentWorkspace Operator"
    );

    let client = Client::try_default().await?;

    if config.leader_election {
        let identity = format!(
            "operator-{}-{}",
            hostname::get()
                .map(|h| h.to_string_lossy().to_string())
                .unwrap_or_else(|_| "node".to_string()),
            uuid::Uuid::new_v4().simple()
        );
        info!(identity = %identity, "Entering leader election loop");

        let leases: Api<Lease> = Api::namespaced(client.clone(), &config.namespace);

        loop {
            match acquire_or_renew_lease(&leases, &config.lease_name, &identity, config.lease_duration).await {
                Ok(is_leader) => {
                    if is_leader {
                        info!(identity = %identity, "Leader election acquired! Starting active controller loop.");
                        // Run active reconciliation until leadership is lost or cancelled
                        run_active_reconciliation(client.clone(), &config.namespace).await?;
                        break;
                    } else {
                        info!("Standby replica waiting for leader lease renewal...");
                        tokio::time::sleep(config.lease_duration / 2).await;
                    }
                }
                Err(err) => {
                    warn!(error = %err, "Leader election probe failed, retrying in 5s");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    } else {
        info!("Leader election disabled; starting active reconciliation directly");
        run_active_reconciliation(client, &config.namespace).await?;
    }

    Ok(())
}

async fn acquire_or_renew_lease(
    leases: &Api<Lease>,
    lease_name: &str,
    holder_identity: &str,
    lease_duration: Duration,
) -> Result<bool> {
    use k8s_openapi::jiff::Timestamp;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;
    let now = Timestamp::now();
    let micro_now = MicroTime(now);

    let lease_patch = serde_json::json!({
        "apiVersion": "coordination.k8s.io/v1",
        "kind": "Lease",
        "metadata": {
            "name": lease_name
        },
        "spec": {
            "holderIdentity": holder_identity,
            "leaseDurationSeconds": lease_duration.as_secs() as i32,
            "acquireTime": micro_now,
            "renewTime": micro_now
        }
    });

    let params = PatchParams::apply("factory-operator").force();
    match leases.patch(lease_name, &params, &Patch::Apply(&lease_patch)).await {
        Ok(_) => Ok(true),
        Err(e) => {
            error!(error = %e, "Failed to apply lease patch");
            Ok(false)
        }
    }
}

async fn run_active_reconciliation(_client: Client, namespace: &str) -> Result<()> {
    info!(namespace = %namespace, "Operator active reconciliation loop initialized");
    // Placeholder for kube::runtime::Controller invocation (wired in T019)
    Ok(())
}
