use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionEnvironment {
    Production,
    Staging,
    Development,
    Test,
}

impl Default for ExecutionEnvironment {
    fn default() -> Self {
        Self::Production
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkloadTrustLevel {
    Untrusted,
    TrustedInternal,
}

impl Default for WorkloadTrustLevel {
    fn default() -> Self {
        Self::Untrusted
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxIsolationType {
    KubernetesGVisor {
        runtime_class: String,
        namespace: String,
    },
    HostSubprocessAllowedOnlyForTrustedDev,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionSecurityPolicy {
    pub environment: ExecutionEnvironment,
    pub trust_level: WorkloadTrustLevel,
    pub isolation: SandboxIsolationType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum SecurityPolicyViolation {
    #[error("Forbidden host execution in production")]
    ForbiddenHostExecutionInProduction,

    #[error("Forbidden host execution for untrusted workload")]
    ForbiddenHostExecutionForUntrustedWorkload,

    #[error("Forbidden isolation downgrade")]
    ForbiddenIsolationDowngrade,

    #[error("Invalid namespace: {0}")]
    InvalidNamespace(String),

    #[error("Missing runtime configuration: {0}")]
    MissingRuntimeConfiguration(String),

    #[error("Execution security policy failed closed: {0}")]
    FailClosed(String),
}

impl Default for ExecutionSecurityPolicy {
    fn default() -> Self {
        Self {
            environment: ExecutionEnvironment::Production,
            trust_level: WorkloadTrustLevel::Untrusted,
            isolation: SandboxIsolationType::KubernetesGVisor {
                runtime_class: "gvisor".to_string(),
                namespace: "factory-sandboxes".to_string(),
            },
        }
    }
}

impl ExecutionSecurityPolicy {
    /// Create a strict default policy for untrusted workloads in production.
    pub fn strict_production() -> Self {
        Self::default()
    }

    /// Evaluates whether the requested execution violates security boundaries.
    pub fn validate(&self) -> Result<(), SecurityPolicyViolation> {
        match (&self.environment, &self.trust_level, &self.isolation) {
            // Production NEVER permits host subprocess execution under any circumstance
            (
                ExecutionEnvironment::Production,
                _,
                SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
            ) => Err(SecurityPolicyViolation::ForbiddenHostExecutionInProduction),
            // Untrusted workloads NEVER permit host subprocess execution in ANY environment
            (
                _,
                WorkloadTrustLevel::Untrusted,
                SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
            ) => Err(SecurityPolicyViolation::ForbiddenHostExecutionForUntrustedWorkload),
            // Kubernetes gVisor isolation checks
            (
                _,
                _,
                SandboxIsolationType::KubernetesGVisor {
                    runtime_class,
                    namespace,
                },
            ) => {
                if runtime_class.trim().is_empty() {
                    return Err(SecurityPolicyViolation::MissingRuntimeConfiguration(
                        "RuntimeClass cannot be empty".to_string(),
                    ));
                }
                if namespace.trim().is_empty() {
                    return Err(SecurityPolicyViolation::InvalidNamespace(
                        "Namespace cannot be empty".to_string(),
                    ));
                }
                Ok(())
            }
            // Host execution only permitted for TrustedInternal workloads strictly in Dev/Test environments
            (
                ExecutionEnvironment::Development | ExecutionEnvironment::Test,
                WorkloadTrustLevel::TrustedInternal,
                SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
            ) => Ok(()),
            // Staging rejects host execution
            (
                ExecutionEnvironment::Staging,
                _,
                SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
            ) => Err(SecurityPolicyViolation::ForbiddenHostExecutionInProduction),
        }
    }

    /// Resolves policy from environment configuration, defaulting to fail-closed Production + Untrusted.
    pub fn from_env() -> Self {
        let env_str = std::env::var("FACTORY_ENV").unwrap_or_else(|_| "production".to_string());
        let environment = match env_str.to_lowercase().as_str() {
            "production" | "prod" => ExecutionEnvironment::Production,
            "staging" | "stage" => ExecutionEnvironment::Staging,
            "development" | "dev" => ExecutionEnvironment::Development,
            "test" => ExecutionEnvironment::Test,
            _ => ExecutionEnvironment::Production, // Fail closed
        };

        let trust_str =
            std::env::var("WORKLOAD_TRUST_LEVEL").unwrap_or_else(|_| "untrusted".to_string());
        let trust_level = match trust_str.to_lowercase().as_str() {
            "trusted" | "trusted_internal" => WorkloadTrustLevel::TrustedInternal,
            _ => WorkloadTrustLevel::Untrusted, // Fail closed
        };

        // If untrusted or production, force Kubernetes gVisor
        let isolation = if environment == ExecutionEnvironment::Production
            || trust_level == WorkloadTrustLevel::Untrusted
        {
            let runtime_class =
                std::env::var("GVISOR_RUNTIME_CLASS").unwrap_or_else(|_| "gvisor".to_string());
            let namespace = std::env::var("SANDBOX_NAMESPACE")
                .unwrap_or_else(|_| "factory-sandboxes".to_string());
            SandboxIsolationType::KubernetesGVisor {
                runtime_class,
                namespace,
            }
        } else {
            let allow_dev_host =
                std::env::var("ALLOW_DEV_HOST_EXECUTION").unwrap_or_else(|_| "false".to_string());
            if allow_dev_host.to_lowercase() == "true" {
                SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev
            } else {
                let runtime_class =
                    std::env::var("GVISOR_RUNTIME_CLASS").unwrap_or_else(|_| "gvisor".to_string());
                let namespace = std::env::var("SANDBOX_NAMESPACE")
                    .unwrap_or_else(|_| "factory-sandboxes".to_string());
                SandboxIsolationType::KubernetesGVisor {
                    runtime_class,
                    namespace,
                }
            }
        };

        Self {
            environment,
            trust_level,
            isolation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy_is_fail_closed_production_gvisor() {
        let policy = ExecutionSecurityPolicy::default();
        assert_eq!(policy.environment, ExecutionEnvironment::Production);
        assert_eq!(policy.trust_level, WorkloadTrustLevel::Untrusted);
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_production_rejects_host_execution() {
        let policy = ExecutionSecurityPolicy {
            environment: ExecutionEnvironment::Production,
            trust_level: WorkloadTrustLevel::TrustedInternal,
            isolation: SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
        };
        assert_eq!(
            policy.validate(),
            Err(SecurityPolicyViolation::ForbiddenHostExecutionInProduction)
        );
    }

    #[test]
    fn test_untrusted_workload_rejects_host_execution_even_in_dev() {
        let policy = ExecutionSecurityPolicy {
            environment: ExecutionEnvironment::Development,
            trust_level: WorkloadTrustLevel::Untrusted,
            isolation: SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
        };
        assert_eq!(
            policy.validate(),
            Err(SecurityPolicyViolation::ForbiddenHostExecutionForUntrustedWorkload)
        );
    }

    #[test]
    fn test_trusted_workload_in_dev_allows_host_execution() {
        let policy = ExecutionSecurityPolicy {
            environment: ExecutionEnvironment::Development,
            trust_level: WorkloadTrustLevel::TrustedInternal,
            isolation: SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev,
        };
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_empty_runtime_class_fails_validation() {
        let policy = ExecutionSecurityPolicy {
            environment: ExecutionEnvironment::Production,
            trust_level: WorkloadTrustLevel::Untrusted,
            isolation: SandboxIsolationType::KubernetesGVisor {
                runtime_class: "  ".to_string(),
                namespace: "factory-sandboxes".to_string(),
            },
        };
        assert!(matches!(
            policy.validate(),
            Err(SecurityPolicyViolation::MissingRuntimeConfiguration(_))
        ));
    }
}
