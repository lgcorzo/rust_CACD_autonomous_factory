# Evaluation Report

## Methodology

Benchmarks are run against real open-source repositories.
Token counts use a consistent `len(text) // 4` approximation.
Impact accuracy reports two ground-truth modes: graph-derived (circular — upper bound) and co-change (files co-changed in the same commit, seed excluded).
Rows with `status=error` are kept for forensics but excluded from all aggregates.
