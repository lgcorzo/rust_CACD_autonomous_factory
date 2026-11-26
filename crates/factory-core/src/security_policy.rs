use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionEnvironment {
    #[default]
    Production,
    Staging,
    Development,
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WorkloadTrustLevel {
    #[default]
    Untrusted,
    TrustedInternal,
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

    #[error("Sandbox bounds violated: {0}")]
    SandboxBoundsViolated(String),

    #[error("A2A policy violation: {0}")]
    A2aPolicyViolation(String),
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

pub const MICRO_SANDBOX_MAX_MEMORY_MIB: u32 = 30;
pub const MICRO_SANDBOX_MAX_CPU_MILLIS: u32 = 250;
pub const MICRO_SANDBOX_GVISOR_OVERHEAD_MEMORY_MIB: u32 = 35;
pub const MICRO_SANDBOX_GVISOR_OVERHEAD_CPU_MILLIS: u32 = 100;
pub const MICRO_SANDBOX_MAX_EPHEMERAL_STORAGE_MIB: u32 = 100;
pub const MICRO_SANDBOX_TTL_SECS: u32 = 60;

pub const COMPILATION_SANDBOX_MAX_MEMORY_MIB: u32 = 512;
pub const COMPILATION_SANDBOX_MAX_CPU_MILLIS: u32 = 500;
pub const COMPILATION_SANDBOX_MAX_EPHEMERAL_STORAGE_MIB: u32 = 1024;
pub const COMPILATION_SANDBOX_TTL_SECS: u32 = 300;

pub const SIDECAR_POLICY_MAX_MEMORY_MIB: u32 = 20;
pub const SIDECAR_POLICY_MAX_CPU_MILLIS: u32 = 100;

/// Dual workload admission profiles enforced by Kubernetes `ValidatingAdmissionPolicy` (`FR-017`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SandboxProfile {
    #[default]
    MicroSandbox,
    CompilationSandbox,
}

impl SandboxProfile {
    /// Canonical label value for `darkgravity.io/sandbox-profile`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MicroSandbox => "micro-sandbox",
            Self::CompilationSandbox => "compilation-sandbox",
        }
    }

    /// Parses a profile label, defaulting fail-closed to `MicroSandbox` when omitted or unknown.
    pub fn from_str_or_default(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "compilation-sandbox" => Self::CompilationSandbox,
            _ => Self::MicroSandbox,
        }
    }

    /// Returns the quantitative resource and lifecycle bounds for this profile.
    pub fn limits(&self) -> SandboxProfileLimits {
        match self {
            Self::MicroSandbox => SandboxProfileLimits {
                profile: Self::MicroSandbox,
                max_memory_mib: MICRO_SANDBOX_MAX_MEMORY_MIB,
                max_cpu_millis: MICRO_SANDBOX_MAX_CPU_MILLIS,
                gvisor_overhead_memory_mib: MICRO_SANDBOX_GVISOR_OVERHEAD_MEMORY_MIB,
                gvisor_overhead_cpu_millis: MICRO_SANDBOX_GVISOR_OVERHEAD_CPU_MILLIS,
                sidecar_max_memory_mib: SIDECAR_POLICY_MAX_MEMORY_MIB,
                sidecar_max_cpu_millis: SIDECAR_POLICY_MAX_CPU_MILLIS,
                max_ephemeral_storage_mib: MICRO_SANDBOX_MAX_EPHEMERAL_STORAGE_MIB,
                ttl_seconds_after_finished: MICRO_SANDBOX_TTL_SECS,
                active_deadline_seconds: MICRO_SANDBOX_TTL_SECS,
                require_gvisor: true,
            },
            Self::CompilationSandbox => SandboxProfileLimits {
                profile: Self::CompilationSandbox,
                max_memory_mib: COMPILATION_SANDBOX_MAX_MEMORY_MIB,
                max_cpu_millis: COMPILATION_SANDBOX_MAX_CPU_MILLIS,
                gvisor_overhead_memory_mib: 0,
                gvisor_overhead_cpu_millis: 0,
                sidecar_max_memory_mib: SIDECAR_POLICY_MAX_MEMORY_MIB,
                sidecar_max_cpu_millis: SIDECAR_POLICY_MAX_CPU_MILLIS,
                max_ephemeral_storage_mib: COMPILATION_SANDBOX_MAX_EPHEMERAL_STORAGE_MIB,
                ttl_seconds_after_finished: COMPILATION_SANDBOX_TTL_SECS,
                active_deadline_seconds: COMPILATION_SANDBOX_TTL_SECS,
                require_gvisor: true,
            },
        }
    }
}

impl std::str::FromStr for SandboxProfile {
    type Err = SecurityPolicyViolation;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "micro-sandbox" => Ok(Self::MicroSandbox),
            "compilation-sandbox" => Ok(Self::CompilationSandbox),
            other => Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Unknown sandbox profile '{}'; expected 'micro-sandbox' or 'compilation-sandbox'",
                other
            ))),
        }
    }
}

/// Quantitative admission limits and TTL ceilings for a `SandboxProfile`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfileLimits {
    pub profile: SandboxProfile,
    pub max_memory_mib: u32,
    pub max_cpu_millis: u32,
    pub gvisor_overhead_memory_mib: u32,
    pub gvisor_overhead_cpu_millis: u32,
    pub sidecar_max_memory_mib: u32,
    pub sidecar_max_cpu_millis: u32,
    pub max_ephemeral_storage_mib: u32,
    pub ttl_seconds_after_finished: u32,
    pub active_deadline_seconds: u32,
    pub require_gvisor: bool,
}

impl SandboxProfileLimits {
    /// Total pod memory ceiling including fixed gVisor isolation overhead (`65 MiB` for micro-sandbox).
    pub fn total_pod_memory_mib(&self) -> u32 {
        self.max_memory_mib + self.gvisor_overhead_memory_mib
    }

    /// Total pod CPU ceiling including fixed gVisor isolation overhead (`350m` for micro-sandbox).
    pub fn total_pod_cpu_millis(&self) -> u32 {
        self.max_cpu_millis + self.gvisor_overhead_cpu_millis
    }

    /// Validates primary container memory, CPU, ephemeral storage, and TTL against profile caps.
    pub fn validate_container_resources(
        &self,
        memory_mib: u32,
        cpu_millis: u32,
        ephemeral_storage_mib: u32,
        ttl_secs: u32,
    ) -> Result<(), SecurityPolicyViolation> {
        if memory_mib == 0 || memory_mib > self.max_memory_mib {
            return Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Profile '{}' memory {}Mi exceeds cap {}Mi",
                self.profile.as_str(),
                memory_mib,
                self.max_memory_mib
            )));
        }
        if cpu_millis == 0 || cpu_millis > self.max_cpu_millis {
            return Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Profile '{}' CPU {}m exceeds cap {}m",
                self.profile.as_str(),
                cpu_millis,
                self.max_cpu_millis
            )));
        }
        if ephemeral_storage_mib == 0 || ephemeral_storage_mib > self.max_ephemeral_storage_mib {
            return Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Profile '{}' ephemeral-storage {}Mi exceeds cap {}Mi",
                self.profile.as_str(),
                ephemeral_storage_mib,
                self.max_ephemeral_storage_mib
            )));
        }
        if ttl_secs == 0 || ttl_secs > self.ttl_seconds_after_finished {
            return Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Profile '{}' ttlSecondsAfterFinished {}s exceeds cap {}s",
                self.profile.as_str(),
                ttl_secs,
                self.ttl_seconds_after_finished
            )));
        }
        Ok(())
    }

    /// Validates sidecar container memory (`<= 20Mi`) and CPU (`<= 100m` / `0.10`) limits.
    pub fn validate_sidecar_resources(
        &self,
        memory_mib: u32,
        cpu_millis: u32,
    ) -> Result<(), SecurityPolicyViolation> {
        if memory_mib == 0 || memory_mib > self.sidecar_max_memory_mib {
            return Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Sidecar memory {}Mi exceeds cap {}Mi",
                memory_mib, self.sidecar_max_memory_mib
            )));
        }
        if cpu_millis == 0 || cpu_millis > self.sidecar_max_cpu_millis {
            return Err(SecurityPolicyViolation::SandboxBoundsViolated(format!(
                "Sidecar CPU {}m exceeds cap {}m",
                cpu_millis, self.sidecar_max_cpu_millis
            )));
        }
        Ok(())
    }
}

pub const A2A_MAX_HOPS: u8 = 3;
pub const A2A_JIT_TTL_SECS: u64 = 300;
pub const A2A_MAX_ZEROIZE_MICROS: u64 = 5;
pub const FORBIDDEN_SELF_EDIT_REPO: &str = "lgcorzo/rust_CACD_autonomous_factory";

/// Zero-trust Agent-to-Agent (A2A) Policy Enforcement Point (PEP) configuration (`FR-006`, `FR-007`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A2aPolicyConfig {
    pub max_hops: u8,
    pub jit_ttl_secs: u64,
    pub max_zeroize_micros: u64,
    pub forbidden_self_edit_repo: String,
}

impl Default for A2aPolicyConfig {
    fn default() -> Self {
        Self {
            max_hops: A2A_MAX_HOPS,
            jit_ttl_secs: A2A_JIT_TTL_SECS,
            max_zeroize_micros: A2A_MAX_ZEROIZE_MICROS,
            forbidden_self_edit_repo: FORBIDDEN_SELF_EDIT_REPO.to_string(),
        }
    }
}

impl A2aPolicyConfig {
    /// Returns `true` when `repo` targets the factory's own protected repository.
    pub fn is_self_edit_forbidden(&self, repo: &str) -> bool {
        repo.trim()
            .eq_ignore_ascii_case(&self.forbidden_self_edit_repo)
    }

    /// Validates A2A delegation depth (`1..=max_hops`) and rejects cyclic caller traces.
    pub fn validate_hop_and_trace(
        &self,
        hop: u8,
        caller: &str,
        trace: &[String],
    ) -> Result<(), SecurityPolicyViolation> {
        if hop == 0 || hop > self.max_hops {
            return Err(SecurityPolicyViolation::A2aPolicyViolation(format!(
                "A2A hop count {} outside allowed range 1..={}",
                hop, self.max_hops
            )));
        }
        let caller_trimmed = caller.trim();
        if caller_trimmed.is_empty() {
            return Err(SecurityPolicyViolation::A2aPolicyViolation(
                "A2A caller identity must not be empty".to_string(),
            ));
        }
        if trace.iter().any(|entry| entry.trim() == caller_trimmed) {
            return Err(SecurityPolicyViolation::A2aPolicyViolation(format!(
                "A2A delegation cycle detected for caller '{}' in trace {:?}",
                caller_trimmed, trace
            )));
        }
        Ok(())
    }

    /// Validates that non-renewable JIT credential TTL is strictly within `1..=jit_ttl_secs`.
    pub fn validate_jit_ttl(&self, ttl_secs: u64) -> Result<(), SecurityPolicyViolation> {
        if ttl_secs == 0 || ttl_secs > self.jit_ttl_secs {
            return Err(SecurityPolicyViolation::A2aPolicyViolation(format!(
                "A2A JIT token TTL {}s violates strict non-renewable bound 1..={}s",
                ttl_secs, self.jit_ttl_secs
            )));
        }
        Ok(())
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

    #[test]
    fn test_dual_sandbox_profiles_and_validation() {
        let micro = SandboxProfile::MicroSandbox.limits();
        assert_eq!(micro.profile, SandboxProfile::MicroSandbox);
        assert_eq!(micro.max_memory_mib, 30);
        assert_eq!(micro.max_cpu_millis, 250);
        assert_eq!(micro.gvisor_overhead_memory_mib, 35);
        assert_eq!(micro.gvisor_overhead_cpu_millis, 100);
        assert_eq!(micro.total_pod_memory_mib(), 65);
        assert_eq!(micro.sidecar_max_memory_mib, 20);
        assert_eq!(micro.sidecar_max_cpu_millis, 100);
        assert_eq!(micro.max_ephemeral_storage_mib, 100);
        assert_eq!(micro.ttl_seconds_after_finished, 60);
        assert!(micro.validate_container_resources(30, 250, 100, 60).is_ok());
        assert!(micro
            .validate_container_resources(31, 250, 100, 60)
            .is_err());
        assert!(micro
            .validate_container_resources(30, 251, 100, 60)
            .is_err());
        assert!(micro
            .validate_container_resources(30, 250, 101, 60)
            .is_err());
        assert!(micro
            .validate_container_resources(30, 250, 100, 61)
            .is_err());
        assert!(micro.validate_sidecar_resources(20, 100).is_ok());
        assert!(micro.validate_sidecar_resources(21, 100).is_err());
        assert!(micro.validate_sidecar_resources(20, 101).is_err());

        let compile = SandboxProfile::CompilationSandbox.limits();
        assert_eq!(compile.profile, SandboxProfile::CompilationSandbox);
        assert_eq!(compile.max_memory_mib, 512);
        assert_eq!(compile.max_cpu_millis, 500);
        assert_eq!(compile.max_ephemeral_storage_mib, 1024);
        assert_eq!(compile.ttl_seconds_after_finished, 300);
        assert!(compile
            .validate_container_resources(512, 500, 1024, 300)
            .is_ok());
        assert!(compile
            .validate_container_resources(513, 500, 1024, 300)
            .is_err());
    }

    #[test]
    fn test_a2a_policy_config_guards() {
        let policy = A2aPolicyConfig::default();
        assert_eq!(policy.max_hops, 3);
        assert_eq!(policy.jit_ttl_secs, 300);
        assert_eq!(policy.max_zeroize_micros, 5);
        assert!(policy.is_self_edit_forbidden("lgcorzo/rust_CACD_autonomous_factory"));
        assert!(!policy.is_self_edit_forbidden("lgcorzo/mlops-python-package"));

        assert!(policy.validate_hop_and_trace(1, "rustant", &[]).is_ok());
        assert!(policy
            .validate_hop_and_trace(
                3,
                "zeroclaw",
                &["rustant".to_string(), "aethalgard".to_string()]
            )
            .is_ok());
        assert!(policy.validate_hop_and_trace(4, "zeroclaw", &[]).is_err());
        assert!(policy
            .validate_hop_and_trace(2, "rustant", &["rustant".to_string()])
            .is_err());
        assert!(policy.validate_jit_ttl(300).is_ok());
        assert!(policy.validate_jit_ttl(0).is_err());
        assert!(policy.validate_jit_ttl(301).is_err());
    }
}
