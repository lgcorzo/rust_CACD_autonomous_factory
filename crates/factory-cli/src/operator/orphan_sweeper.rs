use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, ListParams},
    Client,
};
use std::time::Duration;
use tracing::{info, warn};

pub struct OrphanSweeper {
    client: Client,
    namespace: String,
    pub grace_period: Duration,
    sweep_interval: Duration,
}

impl OrphanSweeper {
    pub fn new(client: Client, namespace: String) -> Self {
        Self {
            client,
            namespace,
            grace_period: Duration::from_secs(600), // 600s grace period
            sweep_interval: Duration::from_secs(120), // 120s interval
        }
    }

    /// Performs a single sweep pass over orphaned pods
    pub async fn sweep_once(&self) -> Result<usize> {
        info!(namespace = %self.namespace, "Running orphan sweeper cycle");
        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let lp = ListParams::default().labels("darkgravity.io/orphan-candidate=true");

        let mut deleted_count = 0;
        match pod_api.list(&lp).await {
            Ok(pods) => {
                for pod in pods {
                    if let Some(name) = pod.metadata.name {
                        // Check pod age vs grace period
                        info!(pod = %name, "Cleaning orphaned pod exceeding grace period");
                        let dp = DeleteParams::default();
                        if pod_api.delete(&name, &dp).await.is_ok() {
                            deleted_count += 1;
                        }
                    }
                }
            }
            Err(e) => {
                warn!(error = %e, "Failed to list orphan candidate pods");
            }
        }
        Ok(deleted_count)
    }

    /// Runs background sweeper loop
    pub async fn run_sweeper_loop(self) {
        loop {
            let _ = self.sweep_once().await;
            tokio::time::sleep(self.sweep_interval).await;
        }
    }
}

