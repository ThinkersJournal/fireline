kind: op_facts v2 fixture and the contract v5 golden pinned
---
`tests/data/kiss_op_facts.json` is now the merged v2 file (KISS main be637a51) and the v5 golden document from `contract_vectors.json` (1349 bytes) is a fixture: it frames, parses at contract_version 5, passes the cross-section rules, and is refused as a v4 document. No reader change was needed; the existing readers already accepted both.
