use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionRecord {
    pub decision_id: String,
    pub agent_id: String,
    pub mission_id: String,
    pub reasoning: String,
    pub ast_node_ids: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MissionPlan {
    pub mission_id: String,
    pub title: String,
    pub spec_content: String,
    pub constitution_rules: Vec<String>,
    pub proposed_tasks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conflict {
    pub conflict_id: String,
    pub rule_violated: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProvenanceReport {
    pub patch_id: String,
    pub is_valid: bool,
    pub causal_chain: Vec<String>,
    pub policy_violations: Vec<String>,
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait SemanticaClient: Send + Sync {
    async fn record_decision(&self, record: &DecisionRecord) -> anyhow::Result<()>;
    async fn detect_conflicts(&self, plan: &MissionPlan) -> anyhow::Result<Vec<Conflict>>;
    async fn verify_provenance(&self, patch_id: &str) -> anyhow::Result<ProvenanceReport>;
}

pub struct HttpSemanticaClient {
    endpoint: String,
    nhi_identity: Option<String>,
    client: reqwest::Client,
}

impl HttpSemanticaClient {
    pub fn new(endpoint: String, nhi_identity: Option<String>) -> Self {
        Self {
            endpoint,
            nhi_identity,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl SemanticaClient for HttpSemanticaClient {
    async fn record_decision(&self, record: &DecisionRecord) -> anyhow::Result<()> {
        let url = format!("{}/v1/decisions", self.endpoint.trim_end_matches('/'));
        let mut req = self.client.post(&url).json(record);

        if let Some(ref nhi) = self.nhi_identity {
            req = req.header("X-NHI-Signature", nhi);
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            anyhow::bail!("Semantica record_decision failed with status: {}", res.status());
        }

        Ok(())
    }

    async fn detect_conflicts(&self, plan: &MissionPlan) -> anyhow::Result<Vec<Conflict>> {
        let url = format!("{}/v1/conflicts/detect", self.endpoint.trim_end_matches('/'));
        let mut req = self.client.post(&url).json(plan);

        if let Some(ref nhi) = self.nhi_identity {
            req = req.header("X-NHI-Signature", nhi);
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            anyhow::bail!("Semantica detect_conflicts failed with status: {}", res.status());
        }

        let conflicts: Vec<Conflict> = res.json().await?;
        Ok(conflicts)
    }

    async fn verify_provenance(&self, patch_id: &str) -> anyhow::Result<ProvenanceReport> {
        let url = format!(
            "{}/v1/provenance/verify/{}",
            self.endpoint.trim_end_matches('/'),
            patch_id
        );
        let mut req = self.client.get(&url);

        if let Some(ref nhi) = self.nhi_identity {
            req = req.header("X-NHI-Signature", nhi);
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            anyhow::bail!("Semantica verify_provenance failed with status: {}", res.status());
        }

        let report: ProvenanceReport = res.json().await?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_semantica_client() {
        let mut mock = MockSemanticaClient::new();
        mock.expect_record_decision()
            .times(1)
            .returning(|_| Ok(()));

        let record = DecisionRecord {
            decision_id: "dec-123".to_string(),
            agent_id: "zeroclaw-01".to_string(),
            mission_id: "mis-456".to_string(),
            reasoning: "Refactored module to reduce cyclomatic complexity".to_string(),
            ast_node_ids: vec!["fn_parse_ast".to_string()],
            timestamp: "2026-08-13T17:00:00Z".to_string(),
        };

        let res = mock.record_decision(&record).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_mock_detect_conflicts() {
        let mut mock = MockSemanticaClient::new();
        mock.expect_detect_conflicts()
            .times(1)
            .returning(|_| {
                Ok(vec![Conflict {
                    conflict_id: "conf-1".to_string(),
                    rule_violated: "constitution_rule_lat_01".to_string(),
                    description: "Proposed design exceeds max response latency".to_string(),
                    severity: "HIGH".to_string(),
                }])
            });

        let plan = MissionPlan {
            mission_id: "m-1".to_string(),
            title: "Add Caching Layer".to_string(),
            spec_content: "High performance cache".to_string(),
            constitution_rules: vec!["max_latency_ms=50".to_string()],
            proposed_tasks: vec!["Deploy Redis".to_string()],
        };

        let conflicts = mock.detect_conflicts(&plan).await.unwrap();
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].severity, "HIGH");
    }
}

