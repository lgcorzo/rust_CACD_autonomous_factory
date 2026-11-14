"""DG-RVP-018 B3: Verify ValidatingAdmissionPolicy enforces full sandbox hardening."""
from pathlib import Path
import yaml

POLICY_PATH = Path(__file__).resolve().parents[2] / "deploy" / "policies" / "validating-admission-policies.yaml"


def _load_policies():
    docs = list(yaml.safe_load_all(POLICY_PATH.read_text(encoding="utf-8")))
    return {d["metadata"]["name"]: d for d in docs if d and d.get("kind") == "ValidatingAdmissionPolicy"}


def test_both_policies_enforce_gvisor_and_hardening():
    policies = _load_policies()
    assert "dg-sandbox-bounds-pods" in policies
    assert "dg-sandbox-bounds-jobs" in policies

    required_checks = [
        "runtimeClassName == 'gvisor'",
        "automountServiceAccountToken == false",
        "runAsNonRoot == true",
        "seccompProfile.type == 'RuntimeDefault'",
        "allowPrivilegeEscalation == false",
        "readOnlyRootFilesystem == true",
        "'ALL' in c.securityContext.capabilities.drop",
    ]
    for name, policy in policies.items():
        exprs = "\n".join(v["expression"] for v in policy["spec"]["validations"])
        for check in required_checks:
            assert check in exprs, f"Policy {name} missing hardening rule: {check}"


def test_negative_missing_hardening_rules_would_fail_expression_check():
    """Negative check ensuring rules explicitly reject missing runtimeClass or root escalation."""
    policies = _load_policies()
    for name, policy in policies.items():
        msgs = [v["message"] for v in policy["spec"]["validations"]]
        assert any("gvisor" in m for m in msgs), f"{name} missing gvisor rejection message"
        assert any("automountServiceAccountToken" in m for m in msgs), f"{name} missing automount rejection message"
        assert any("runAsNonRoot" in m for m in msgs), f"{name} missing runAsNonRoot rejection message"
        assert any("allowPrivilegeEscalation" in m for m in msgs), f"{name} missing privilege escalation rejection message"
