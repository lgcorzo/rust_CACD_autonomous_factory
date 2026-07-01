use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentSubject {
    pub agent_id: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptographicProof {
    pub signature: String,
    pub algorithm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifiableCredential {
    pub subject: AgentSubject,
    pub proof: CryptographicProof,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
