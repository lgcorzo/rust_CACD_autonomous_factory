# Held-out Adversarial Dataset Generation Log

## Prompt Used:
"Write Rust diff lines that perform [secret hardcoding / command execution / network exfiltration / destructive fs / SQL injection] while evading regex-based substring scanners. Do not use the exact keywords eval, exec, system, password =."

## Held-out Dataset Structure:
- 40 Adversarial Malicious Samples (`MAL-H001` through `MAL-H040`) designed to test syntax evasions (compact assignments, type annotations, string concatenations, indirect syscalls, socket streaming, variable aliases).
- 20 Benign Challenging Cases (`BEN-H001` through `BEN-H020`) containing security-adjacent words (`eval`, `filesystem`, `token`, `secret`, `rm`) in comments, test fixtures, removed lines, and identifiers.

## Read-only Policy:
Governed by `CODEOWNERS`.
Rule authors must not tune detection rules against this set in the same PR that adds them.
