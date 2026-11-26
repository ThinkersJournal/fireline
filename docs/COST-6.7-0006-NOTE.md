# What KISS-Contract 6.7-0006 (cost) would need from fireline-contract

Date 2026-11-26. Lane note, **no code until KISS settles scope** (pm-kernels TASK). Read at KISS `origin/main` 153c09a (`spec/contract.md`) and fireline `origin/main` 3bfe2b9. Source of the open question: KISS-Contract #125 (OPEN), read whole 2026-11-26.

## Where the clause stands

- **6.7-0006 as pinned:** Capabilities `cost` carries a cost **class** plus cost **expressions** in the 6.6-0006 grammar over the launch-scalar symbols; no bare per-element scalar; no symbol outside the vocabulary; test `test_contract_cost_expressions`. 6.8-0006 makes `cost_provenance` (`declared` or `measured`) mandatory "where a cost is stated" and mirrored in Provenance (6.9-0006). 6.8-0007 makes `cost` the single home of magnitude.
- **#125 asks** (Fuel): cost as a small vector of expressions (compute, bandwidth, overhead, per-tier memory; axes not fixed), same grammar, arity only; a single expression stays legal and reads as the compute term; `cost_provenance` possibly landed alone. Dispositions so far: KISS lane "proposed: declined, pending Fuel" (2026-11-03), Fuel "restating, not withdrawing" (2026-11-04, with the consumer behaviour that needs separate axes). **No editor ruling is in the thread; scope is unsettled.**

## What fireline-contract does today (read at 3bfe2b9)

- `cost` is a schema field only (`fields.rs`, Capabilities, required): its value is **not parsed** and **no rule reads it**; `consistency.rs` says so ("Not here: the cost expressions (6.7-0006, 6.8-0007)").
- `cost_provenance` **is** checked: in the `declared`/`measured` vocabulary, and Guarantees equals Provenance (6.8-0006, 6.9-0006).
- The 6.6-0006 expression parser exists (`Expr::parse_in` with a `Context`), used for Dispatch fields, so a cost expression can be parsed with the same code.

**Update 2026-11-26 (pm-kernels NOTICE, KISS at 153c09a, contract.md:2160 and :224, ROADMAP R1.25):** keep `cost = 1` as it is; the cost-class set and the value syntax are **unspecified until R1.25 is ruled** (needs CireSnave's verbatim word); this lane mints no cost outcomes. The table below is therefore a list of what each KISS outcome *would* need, not a plan, and nothing in it is scheduled.

## What each outcome of #125 would need (decision table, no code)

| KISS outcome | fireline-contract work | Needs from KISS first |
|---|---|---|
| **Declined; scalar-valued cost stays** | parse `cost` as `<class> <expr>` and run 6.7-0006's three MUSTs: an expression in the grammar, symbols inside the launch-scalar vocabulary, no bare per-element scalar | the **value syntax** of `cost` (how class and expression are spelled), the **closed set of cost classes** (the example shows `elementwise`), and vectors; today none of the three is stated: the prose example spells "cost class `elementwise`, cost `1 * n`", but the serialized example in the same spec (line 2160) is `cost = 1`, a bare scalar with no class, and a grep for "cost class" finds no enumeration of classes (read 2026-11-26) |
| **Vector adopted** (pinned field changed) | the same, with N expressions; a single expression read as the compute term; arity and axis names checked | the axis set and its order, the value syntax, and a contract version bump (the field changes meaning, so contract 6 would carry it; v4 and v5 keep the scalar reading) |
| **Optional sibling field** | a new optional trailing Capabilities field, like `requires_co_resident_launch` (6.7-0007, pinned after `cost`), with the same absent-means-default pattern | its name, position (the "pinned last" rule makes ordering a schema question) and absent meaning |
| **`cost_provenance` alone** | nothing: it is already enforced | nothing |

## What does not wait on KISS

Nothing here can be built honestly now. The decisive missing input is the same in every row: **the value syntax of `cost` and its vectors**. Without a KISS corpus vector, any reader this crate wrote would be written from the clause text only ("ours", in the ROADMAP's sense, not "diffed"). **Do not build it from a guess**; the held status in ROADMAP stays.

## Two things to ask KISS when it answers

1. Does 6.7-0006's "cost class" have a closed token set, and where is it written? The prose example uses `elementwise`; a reader needs the set to reject an unknown class (6.1: unknown token is a typed decline).
2. Which spelling of `cost` is normative on the wire? The serialized example is `cost = 1`, which 6.7-0006 appears to forbid ("MUST NOT carry a bare per-element scalar"); fireline's own golden vectors use `cost = 1` too, so they would fail a real 6.7-0006 reader. That is a finding for KISS, not a change here.

— lane: fireline
